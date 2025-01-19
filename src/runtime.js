const { core } = Deno;

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
    return core.ops.op_read_file(path);
  },
  writeFile: (path, contents) => {
    return core.ops.op_write_file(path, contents);
  },
  removeFile: (path) => {
    return core.ops.op_remove_file(path);
  },
  fetch: async (url) => {
    return core.ops.op_fetch(url);
  },
};

globalThis.report = () => {
  core.print(`REPORT_CORE: ${argsToMessage(Object.keys(core))}\n`);
  core.print(`REPORT_OPS: ${argsToMessage(Object.keys(core.ops))}\n`);
};

globalThis.setTimeout = async (callback, delay) => {
  core.ops.op_set_timeout(delay).then(callback);
};
