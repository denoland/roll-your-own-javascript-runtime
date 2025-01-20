import { core } from "ext:core/mod.js";
// It's significant that the ops import works this way.
// 1. deno_runtime's main.js removes all ops under Deno.core.ops, so
//    we can't get op_bark from there.
//    See: https://github.com/denoland/deno/discussions/23248#discussioncomment-11890567
// 2. we can't `import { op_bark } from "ext:core/ops";` because, on
//    deno_runtime 0.180.0, we get a runtime error that the module does
//    not have an export named `op_bark`.
// 3. we can't `import ops from "ext:core/ops";` because, on deno_runtime 0.180.0
//    we get a runtime error that the module does not have a default export.
import * as core_ops from "ext:core/ops";
const { op_bark, op_register_task, op_get_next_task_id } = core_ops;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.WORKER_ID = null;

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

globalThis.bark = () => {
  op_bark();
};

globalThis.setTimeout = async (callback, delay) => {
  core.ops.op_set_timeout(delay).then(callback);
};

globalThis.TASKS = {};

globalThis.registerTask = (id, callback) => {
  op_register_task(id);
  globalThis.TASKS[id] = callback;
};

function getNextTaskId() {
  return op_get_next_task_id();
}

globalThis.runAllTasks = async () => {
  // Get the next task by running an op that returns the next task id to run.
  // The look up just that one.
  let cb, taskId;

  while ((taskId = getNextTaskId())) {
    cb = TASKS[taskId];
    delete TASKS[taskId];

    if (cb) {
      await cb();
    }
  }
};
