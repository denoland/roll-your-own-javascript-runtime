const mainTasks = ["alpha", "beta"];

const subTasks = [];

for (let i = 0; i < 2; i++) {
  subTasks.push(i);
}

for (const mainTask of mainTasks) {
  for (const subTask of subTasks) {
    const taskId = `${mainTask}-${subTask}`;
    registerTask(taskId, async () => {
      console.log(`[worker ${WORKER_ID}] performing task ${taskId}`);
    });
  }
}

await runAllTasks();
