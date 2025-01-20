export function registerWorkerId(id) {
  if (id) {
    console.log(`[worker] registerWorkerId received id: ${id}`);
  } else {
    console.log("[worker] registerWorkerId received NO id");
  }
}

console.log("Will bark...");
bark();
