# rattata

<img src="logo.png" alt="rattata" align="right" />

This is the trainer/rattata (payload) system for [pakemon](https://github.com/notnullgames/pakemon).

To get started, install [rustup](https://rustup.rs/). You should also run `cargo install cross`. You will also need docker & make installed.


## NOTES

- runtime has a reserved string for the onion address of the manager. leave off `.onion`. It's 56 characters long, padded with spaces, and you can find it with `ONION_ADDRESS` in the binary. This is how the manager address is hard-coded without re-compiling. Another idea: use filename or even CLI param to set call-back address. BadUSB script could account for param, installing it stealth, etc.

## TODO

This is basically just `helloworld`, now.

- create tor tunnel, runtime connects to manager on startup with basic commands (`download` & `exec`)
- write lots of script payloads for persistance, stealth install, etc
- process management: it should be able to spawn multiple subshells, list, and kill them