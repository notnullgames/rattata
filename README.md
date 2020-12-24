# rattata

![rattata](https://cdn.bulbagarden.net/upload/thumb/4/46/019Rattata.png/500px-019Rattata.png)

lua-based Command and Control framework

This is the server + framework for post-exploitation.

# WIP

This is in the works, and not at all complete.


## development

This is meant to run inside a [luapower](https://luapower.com/) dir-tree.

You can set it up like this:

- Get luapower. Luapower has a few other ways to download the tree (I like using the `mgit` method) so do that however you like. An easy way is to download [a big zip](https://github.com/luapower/all/archive/master.zip) of everything.
- inside that dir, run `git clone https://github.com/notnullgames/rattata.git` or extract [this zip](https://github.com/notnullgames/rattata/archive/main.zip) to a rattata dir inside the tree.


You can run it (for dev) with this:

```
./luajit rattata/main.lua
```

To make payloads for different operating systems:

```
./mgit bundle -z -a --all -m --all -M rattata.main -o dist/linux/x64/rattata
```

Checkout [the docs](https://luapower.com/bundle) for more info about bundling. Eventually, I will setup tuned targets for differnt OS's.