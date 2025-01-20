# `runjs`

# Run Something

```
cargo run ./bark.js
```

```shellsession
$ cargo run ./bark.js
   Compiling runjs v0.1.0 (/Users/mike/repos/roll-your-own-javascript-runtime)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.65s
     Running `target/debug/runjs ./bark.js`
[out]: "[worker: 5nfOqsu] Will bark..."
woof
[out]: "[worker: FgNa4qv] Will bark..."
[out]: "[worker: ZhzmWPl] Will bark..."
[out]: "[worker: ge7ahCK] Will bark..."
woof
woof
woof
worker FgNa4qv finished
worker ge7ahCK finished
worker ZhzmWPl finished
worker 5nfOqsu finished
```

Go to the original README [here](https://github.com/denoland/roll-your-own-javascript-runtime/blob/main/README.md).

This forked repo builds up the original [roll-your-own-javascript-runtime](https://github.com/denoland/roll-your-own-javascript-runtime) by:
1. Using `deno_runtime::MainWorker` instead of `deno_core::JsRuntime`, because the higher level worker provides more capability, such as node built-ins.
2. Making it multi-threaded, so that multiple scripts can be run in parallel, registering tasks that are run uniquely.
3. Demonstrating how to use the node built-ins and NPM packages via pre-bundling.

It also removes snapshotting, because it didn't seem to work any more after the move to `MainWorker`.
Maybe it could be figured out and re-added.

# Use Node Built-ins

The embedded Deno runtime includes the node built-ins. So just prefix those imports with `node:`.

For example, using the `path` module:

```javascript
import path from 'node:path'
const p = path.resolve(path.join('/etc', 'hosts'))
console.log(`path: ${p}`)
```

This can be run with:

```
cargo run ./nodeApiUsage.js
```

# Use NPM Packages

The embedded Deno runtime in this repo does not include support for NPM package resolution. That is a capability of the deno CLI.
But the `deno_runtime` crate upon which this embedded runtime is based does not include support for NPM packages itself, and
currently there's not a great way to access those capabilities of the deno CLI without forking it.

Instead, we can pre-bundle NPM packages into a single file, and then use that file in our embedded runtime.

Deno's documentation provides a recommendation for how to [accomplish pre-bundling here](https://docs.deno.com/runtime/reference/migration_guide/#cli-changes).

That's been set up in this repo as `bundle.js`.

So:

```
npm install
```

to install the dependencies, and then:

```
deno -A bundle.js
```

This creates the bundles under the `bundles/`

Then you can directly execute each bundle using the embedded runtime in this repo, like this:

```
cargo run ./bundles/npmUsage.js
```

# Multi-threading Task Runner

The embedded runtime provides the following APIs for running tasks:

- `WORKER_ID`: a globally available unique ID for the current worker.
- `registerTask(id, callback)`: register a callback to be run when the task with the given task ID is run.
- `runAllTasks()`: run all tasks as they are assigned by the runtime to the current worker.

There is no sharing of data across the `JsRuntime` workers. Every worker registers all of the possible tasks it might run.
It's expected that every worker registers all of the same tasks using the same task IDs, which are generated as descriptive identifiers
of the work to be done for each task.

The embedded runtime runs each worker `JsRuntime` in a separate thread.

Each worker registers all of the tasks.

Then each worker calls `runAllTasks()`, which will get the next available task ID, run it, and repeat until there are no more tasks to run.

While all workers declare the same tasks, only one worker will run each task, because once a given task ID is removed from the set of tasks,
it will not be assigned again. Thus, while a worker registers all tasks that it might be called upon to complete, it will only run
the subset of tasks assigned to it by the runtime.

Thus, it's recommended that expensive computations or memory allocations are done only in the task callbacks.
This allows them to be lazily computed, and only computed once.
