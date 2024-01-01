# `runjs`

A repository for the Roll your own JavaScript runtime blog post series:

- [Roll your own JavaScript runtime](https://deno.com/blog/roll-your-own-javascript-runtime)
- [Roll your own JavaScript runtime, pt. 2](https://deno.com/blog/roll-your-own-javascript-runtime-pt2)
- [Roll your own JavaScript runtime, pt. 3](https://deno.com/blog/roll-your-own-javascript-runtime-pt3)

## UPDATE 2023-04-26

This repo has been updated to use snapshotting to speed up startup times.

[![Andy and Leo add snapshots to speed up startup times for a custom JavaScript runtime](https://i.imgur.com/E9vFzhu.png)](https://www.youtube.com/watch?v=zlJrMGm-XeA)
_Watch the corresponding video._

## UPDATE 2023-02-07

This repo has been updated to support loading JavaScript and TypeScript files.

```shellsession
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/runjs ./test.js`
[out]: "Hello" "runjs!"
[err]: "Boom!"
[err]: "Unable to read file" "./log.txt" {"code":"ENOENT"}
[out]: "Read from a file" "./log.txt" "contents:" "I can write to a file."
[out]: "Removing file" "./log.txt"
[out]: "File removed"
```

[![Andy and Bartek add TypeScript support to a custom JavaScript runtime on YouTube](https://deno.com/blog/roll-your-own-javascript-runtime-pt2/roll-own-js-runtime-screencap.png)](https://www.youtube.com/watch?v=-8L3_OOeENo)
_Watch the corresponding video._
