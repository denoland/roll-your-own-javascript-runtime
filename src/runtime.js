const { core } = Deno;
const { ops } = core;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
  log: (...args) => {
    core.print(`[out]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[err]: ${argsToMessage(...args)}\n`, true);
  },
};

const runjs = {
  readFile: (path) => {
    return ops.op_read_file(path);
  },
  writeFile: (path, contents) => {
    return ops.op_write_file(path, contents);
  },
  removeFile: (path) => {
    return ops.op_remove_file(path);
  },

  fetch: async (url) => {
    return ops.op_fetch(url);
  },
};

globalThis.setTimeout = (callback, delay) => {
  ops.op_set_timeout(delay).then(callback);
};
globalThis.console = console;
globalThis.runjs = runjs;
