# rattata

![rattata](./logo.png)

This is some ideas round using rust for [rattata](https://github.com/notnullgames/rattata).

To get started, install [rustup](https://rustup.rs/). You should also run `cargo install cross`. You will also need docker & make installed.


## NOTES

- runtime has a reserved string for the onion address of the manager. leave off `.onion`. It's 56 characters long, padded with spaces, and you can find it with `ONION_ADDRESS` in the binary. This is how the manager address is hard-coded without re-compiling.

## TODO

This is basically just `helloworld`, now.

- create tor tunnel, runtime connects to manager on startup with basic commands (`download` & `exec`)
- add VFS to store downloaded files & add vfs file management commands (`ls`, `rm`, `cat`) - maybe no dirs, just flat location
- add a way to inject initial VFS & config (like zip at end of binary)
- add advanced runtime commands (load this dll, do stuff with it, etc)
- runtime persistence & hiding
- setup makefile to only use docker (so no rust toolchain needed.)