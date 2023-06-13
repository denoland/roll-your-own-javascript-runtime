use deno_core::extension;
use std::env;
use std::path::PathBuf;

fn main() {
    extension!(
        runjs,
        js = ["src/runtime.js",]
    );

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

    let _snapshot = deno_core::snapshot_util::create_snapshot(
        deno_core::snapshot_util::CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            snapshot_path,
            startup_snapshot: None,
            extensions: vec![runjs::init_js_only()],
            compression_cb: None,
            snapshot_module_load_cb: None,
        }
    );
}
