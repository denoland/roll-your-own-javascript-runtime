((globalThis) => {
  const core = Deno.core;
  // Note: Do not call this when snapshotting, it should be called
  // at runtime. This example does not use V8 snapshots.
  core.initializeAsyncOps();

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
  };

  globalThis.runjs = {
    readFile: (path) => {
      return core.opAsync("op_read_file", path);
    },
    writeFile: (path, contents) => {
      return core.opAsync("op_write_file", path, contents);
    },
    removeFile: (path) => {
      return core.opSync("op_remove_file", path);
    },
  };
})(globalThis);
