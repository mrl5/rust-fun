# Rust fun

My Rust journey started because of [this
tweet](https://twitter.com/JavaScriptDaily/status/1466594793019125769) leading
to incredible [24 days from node.js to Rust](https://vino.dev/blog/node-to-rust-day-1-rustup/) series by [Vino](https://vino.dev/). Also this article got my attention: https://www.zdnet.com/article/rust-takes-a-major-step-forward-as-linuxs-second-official-language/

In parallel I took the [Rust track](https://exercism.org/tracks/rust) with
[exercism](https://exercism.org/).

## Other notable refs
1. https://doc.rust-lang.org/book/
2. https://doc.rust-lang.org/reference/index.html
3. https://doc.rust-lang.org/nomicon/index.html
4. https://rust-unofficial.github.io/patterns/
5. https://cheats.rs/
6. https://rustwasm.github.io/docs/book/

## GOTCHAs
```bash
$ cargo install cargo-edit
(...)

error: failed to run custom build command for `log v0.4.14`

Caused by:
  could not execute process
  `/tmp/cargo-installbVf8st/release/build/log-78c41bf7c7acfe0a/build-script-build`
  (never executed)

  Caused by:
    Permission denied (os error 13)
    warning: build failed, waiting for other jobs to finish...
```

caused by `noexec`:
```bash
$ cat /etc/fstab | grep noexec
tmpfs   /tmp    tmpfs   defaults,noexec,nosuid 0 0
```

solution (https://github.com/rust-lang/cargo/issues/4350#issuecomment-340215811):
```bash
$ TMPDIR=$XDG_RUNTIME_DIR cargo install cargo-edit
```
