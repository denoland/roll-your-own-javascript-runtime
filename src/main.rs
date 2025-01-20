use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleSourceCode;
use deno_fs::RealFs;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::WorkerExecutionMode;
use std::env;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub enum Operation {
  NotifyStart(Sender<Result<(), deno_core::anyhow::Error>>),
  NotifyDone(Sender<Result<(), deno_core::anyhow::Error>>),
}

pub struct AsyncRuntimeHandle {
  pub runtime: Arc<RwLock<Runtime>>,
  pub operation_sender: Sender<Operation>,
}

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
  let contents = tokio::fs::read_to_string(path).await?;
  Ok(contents)
}

#[op2(async)]
async fn op_write_file(
  #[string] path: String,
  #[string] contents: String,
) -> Result<(), AnyError> {
  tokio::fs::write(path, contents).await?;
  Ok(())
}

#[op2(async)]
#[string]
async fn op_fa_fetch(#[string] url: String) -> Result<String, AnyError> {
  let body = reqwest::get(url).await?.text().await?;
  Ok(body)
}

#[op2(async)]
async fn op_set_timeout(delay: f64) -> Result<(), AnyError> {
  tokio::time::sleep(std::time::Duration::from_millis(delay as u64)).await;
  Ok(())
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
  std::fs::remove_file(path)?;
  Ok(())
}

#[op2(fast)]
fn op_bark() {
  println!("woof");
}

struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
  fn resolve(
    &self,
    specifier: &str,
    referrer: &str,
    _kind: deno_core::ResolutionKind,
  ) -> Result<deno_core::ModuleSpecifier, AnyError> {
    deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
  }

  fn load(
    &self,
    module_specifier: &deno_core::ModuleSpecifier,
    _maybe_referrer: Option<&reqwest::Url>,
    _is_dyn_import: bool,
    _requested_module_type: deno_core::RequestedModuleType,
  ) -> ModuleLoadResponse {
    let module_specifier = module_specifier.clone();

    let module_load = move || {
      let path = module_specifier.to_file_path().unwrap();

      let media_type = MediaType::from_path(&path);
      let (module_type, should_transpile) = match MediaType::from_path(&path) {
        MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
          (deno_core::ModuleType::JavaScript, false)
        }
        MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
        MediaType::TypeScript
        | MediaType::Mts
        | MediaType::Cts
        | MediaType::Dts
        | MediaType::Dmts
        | MediaType::Dcts
        | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
        MediaType::Json => (deno_core::ModuleType::Json, false),
        _ => panic!("Unknown extension {:?}", path.extension()),
      };

      let code = std::fs::read_to_string(&path)?;
      let code = if should_transpile {
        let parsed = deno_ast::parse_module(ParseParams {
          specifier: module_specifier.clone(),
          text: code.into(),
          media_type,
          capture_tokens: false,
          scope_analysis: false,
          maybe_syntax: None,
        })?;
        parsed
          .transpile(&Default::default(), &Default::default())?
          .into_source()
          .source
      } else {
        code.into_bytes()
      };
      let module = deno_core::ModuleSource::new(
        module_type,
        ModuleSourceCode::Bytes(code.into_boxed_slice().into()),
        &module_specifier,
        None,
      );
      Ok(module)
    };

    ModuleLoadResponse::Sync(module_load())
  }
}

//static RUNTIME_SNAPSHOT: &[u8] =
//  include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));

extension!(
  runjs,
  ops = [
    op_read_file,
    op_write_file,
    op_remove_file,
    op_fa_fetch,
    op_set_timeout,
    op_bark
  ],
  // list of all JS files in the extension
  esm_entry_point = "ext:runjs/src/runtime.js",
  // the entrypoint to our extension
  esm = ["src/runtime.js"]
);

fn build_worker(main_module: &ModuleSpecifier) -> Result<MainWorker, AnyError> {
  let fs = Arc::new(RealFs);
  let permission_desc_parser =
    Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

  let bootstrap = BootstrapOptions {
    mode: WorkerExecutionMode::Run,
    ..Default::default()
  };

  let worker = MainWorker::bootstrap_from_options(
    main_module.clone(),
    WorkerServiceOptions {
      module_loader: Rc::new(TsModuleLoader),
      permissions: PermissionsContainer::allow_all(permission_desc_parser),
      blob_store: Default::default(),
      broadcast_channel: Default::default(),
      feature_checker: Default::default(),
      node_services: Default::default(),
      npm_process_state_provider: Default::default(),
      root_cert_store_provider: Default::default(),
      shared_array_buffer_store: Default::default(),
      compiled_wasm_module_store: Default::default(),
      v8_code_cache: Default::default(),
      fs,
    },
    WorkerOptions {
      extensions: vec![runjs::init_ops_and_esm()],
      //startup_snapshot: Some(RUNTIME_SNAPSHOT),
      startup_snapshot: None,
      bootstrap,
      skip_op_registration: false,
      ..Default::default()
    },
  );

  Ok(worker)
}

fn run_js(file_path: String) -> Result<(), AnyError> {
  let (tx, mut rx): (Sender<Operation>, Receiver<Operation>) = channel(64);

  let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("could not build tokio runtime");

  let runtime = Arc::new(RwLock::new(runtime));

  let runtime_copy = Arc::clone(&runtime);

  // Launch a new thread for running the Tokio runtime and Worker operations
  let handle = thread::spawn(move || {
    let rt = runtime_copy
      .read()
      .expect("could not get read lock for runtime");

    // Block on the async code
    rt.block_on(async {
      let main_module = deno_core::resolve_path(
        file_path,
        env::current_dir()
          .expect("failed getting current_dir")
          .as_path(),
      )
      .expect("failed resolving path");
      let mut worker =
        build_worker(&main_module).expect("failed initializing worker");

      while let Some(message) = rx.recv().await {
        match message {
          Operation::NotifyStart(response_channel) => {
            worker
              .execute_main_module(&main_module)
              .await
              // TODO: send a message back for this case.
              .expect("failed executing main module");

            let result = worker.run_event_loop(false).await;
            response_channel
              .send(result)
              .await
              .expect("failed sending result response");
          }
          Operation::NotifyDone(_response_channel) => {
            // TODO: make this real
            ()
          }
        }
      }
    });
  });

  let rt = runtime.read().expect("could not get read lock on runtime");

  rt.spawn(async move {
    let (notify_start_response_tx, mut notify_start_response_rx): (
      Sender<Result<(), deno_core::anyhow::Error>>,
      Receiver<Result<(), deno_core::anyhow::Error>>,
    ) = channel(64);

    tx.send(Operation::NotifyStart(notify_start_response_tx))
      .await
      .expect("failed sending start message");

    while let Some(message) = notify_start_response_rx.recv().await {
      match message {
        Ok(()) => {
          println!("worker finished");
        }
        Err(e) => {
          eprintln!("worker error: {:?}", e);
        }
      }
    }
  });

  handle.join().map_err(|e| AnyError::msg(format!("{:?}", e)))
}

fn main() {
  let args = &env::args().collect::<Vec<String>>()[1..];

  //println!("OUT_DIR: {}", env!("OUT_DIR"));

  if args.is_empty() {
    eprintln!("Usage: runjs <file>");
    std::process::exit(1);
  }
  let file_path = &args[0];

  // let runtime = tokio::runtime::Builder::new_current_thread()
  //   .enable_all()
  //   .build()
  //   .unwrap();
  if let Err(error) = run_js(file_path.to_string()) {
    eprintln!("error: {error}");
  }
}
