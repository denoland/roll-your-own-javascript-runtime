# `runjs`

A repository for the [Roll your own JavaScript runtime](https://deno.com/blog/roll-your-own-javascript-runtime) blog post.

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