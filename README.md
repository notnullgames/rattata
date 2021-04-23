# rattata

<img src="logo.png" alt="rattata" align="right" />

This is the trainer/rattata (payload) system for [pakemon](https://github.com/notnullgames/pakemon). See [this](https://github.com/notnullgames/pakemon/wiki/Projects) for a full breakdown of the Pakémon ecosystem.

To get started, install [rustup](https://rustup.rs/). You should also run `cargo install cross`. You will also need docker & make installed.


## NOTES

- runtime has a reserved string for the onion address of the manager. leave off `.onion`. It's 56 characters long, padded with spaces, and you can find it with `ONION_ADDRESS` in the binary. This is how the manager address is hard-coded without re-compiling. Another idea: use filename or even CLI param to set call-back address. BadUSB script could account for param, installing it stealth, etc.

## TODO

This is basically just `helloworld`, now.

- create tor tunnel, runtime connects to manager on startup with basic commands (`download` & `exec`)
- write lots of script payloads for persistance, stealth install, etc
- process management: it should be able to spawn multiple subshells, list, and kill them


## payload stuff

- need to run intiial command stealth. in rust, this can be done like [this](https://stackoverflow.com/questions/29763647/how-to-make-a-program-that-does-not-display-the-console-window). This could be good for intitial payload that can spawn 'powershell -windowstyle hidden -command YOURCOMMANDS'
- [this](https://github.com/cfsamson/powershell-script) might be helpful for inline powershell scripts
- use a [template lib](https://blog.logrocket.com/top-3-templating-libraries-for-rust/) to generate install badusb that types in silent commands for loading initial payload (tor+socket, connect to trainer, accept commands)
- trainer should watch for things like "new rattata" and send it commands to collect info, persist, etc.
- [this](https://github.com/arrase/Raspiducky) seems to have the most options, and do the most
