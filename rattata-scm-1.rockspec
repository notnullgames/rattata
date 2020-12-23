package = "rattata"
version = "scm-1"

source = {
   url = "https://github.com/notnullgames/rattata.git"
}

description = {
   detailed = "![rattata](https://cdn.bulbagarden.net/upload/thumb/4/46/019Rattata.png/500px-019Rattata.png)",
   homepage = "https://github.com/notnullgames/rattata",
   license = "MIT"
}

dependencies = {
   "lua >= 5.1, < 5.4",
   "http",
   "luasocket",
   "cqueues"
}

build = {
   type = "builtin",
   modules = {
      rattata = "src/rattata.lua"
   },
   
   install = {
      bin = {
        "src/rattata.lua"
      }
   }
}

