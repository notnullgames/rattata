NAME=rattata

.PHONY: help clean manager target

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

trainer: ## run rattata manager (trainer)
	cargo run --bin trainer

rattata: ## run rattata target runtime
	cargo run --bin rattata gymsp6larssfbxrhqmmjg274znaptpzaareglv5wvkgiunpoxfajspid.onion:18414

clean: ## delete all output files
	cargo clean

build: ## build runtime files for current system in target/release
	# whatever is local
	cargo build --bins --release

# TODO: handle release stuff here?
