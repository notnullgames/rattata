.PHONY: help setup run

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Install dependencies. Reuiqres luajit and luarocks installed
	sudo luarocks install luasocket luapak

run: ## run rattata service
	@luajit src/rattata.lua

