SHELL != /bin/bash
.PHONY: help


help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean up the build directory project using cargo
	#rm -rf build
	cargo clean

build: ## Build the project using cargo
	cargo build



run: ## Run the project using cargo
	cargo run

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version of the project using cargo
	@echo "Curent Version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter the new version: " version; \
	update_version="$$version" cargo pkgid | cut -d# -f2 | sed - E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo  "New version is $(shell cargo pkgid | cut -d# -f2)"%

