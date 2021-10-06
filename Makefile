prepare:
	rustup target add wasm32-unknown-unknown

build-erc20:
	cd contracts/picas_token && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contracts/picas_token/target/wasm32-unknown-unknown/release/picas_token.wasm 2>/dev/null | true

test: build-erc20
	mkdir -p tests/wasm
	cp contracts/picas_token/target/wasm32-unknown-unknown/release/picas_token.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contracts && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contracts && cargo fmt
	cd tests && cargo fmt

clean:
	cd contracts && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
