use deno_core::extension;
use std::env;
use std::path::PathBuf;

fn main() {
  extension!(
    // extension name
    runjs,
    // list of all JS files in the extension
    esm_entry_point = "ext:runjs/src/runtime.js",
    // the entrypoint to our extension
    esm = ["src/runtime.js"]
  );

  let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
  let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

  let snapshot_options =
    deno_runtime::ops::bootstrap::SnapshotOptions::default();

  //vec![runjs::init_ops_and_esm()],
  deno_runtime::snapshot::create_runtime_snapshot(
    snapshot_path,
    snapshot_options,
    vec![runjs::init_ops_and_esm()],
  );
}
