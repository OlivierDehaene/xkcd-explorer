run: ## Run rust binary
	./target/release/xkcd-explorer-server

build: ## Build Rust binary
	cargo build --release

rustup: ## Install Repository dependencies
	curl https://sh.rustup.rs -sSf | sh -s -- -y

format: ## Format code
	cargo fmt --all

format-check: ## Check that code is properly formatted
	cargo fmt --all -- --check

lint: ## Lint code
	cargo clippy --workspace

lint-check: ## Check that code is properly linted
	cargo clippy --workspace -- -D warnings