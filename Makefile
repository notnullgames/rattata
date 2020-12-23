.PHONY: help setup run

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Install dependencies. Requires luarocks installed.
	sudo luarocks install luasocket
	sudo luarocks install luapak

run: ## Run rattata service
	@luajit src/rattata.lua

