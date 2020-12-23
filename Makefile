.PHONY: help setup run clean

CFLAGS=$(shell pkg-config --cflags --libs --static lua)

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Install dependencies. Requires luarocks installed.
	sudo luarocks install luasocket
	sudo luarocks install http
	sudo luarocks install luapak

run: ## Run rattata service
	@luajit src/rattata.lua

build: dist/rattata ## compile standalone payload

clean: # delete generated files
	rm -rf dist .luapak

dist/rattata:
	luapak make --lua-lib=/usr/lib/x86_64-linux-gnu/libluajit-5.1.so
