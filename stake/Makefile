prepare:
	rustup target add wasm32-unknown-unknown

build-staking:
	cd contracts && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contracts/target/wasm32-unknown-unknown/release/staking_contract.wasm 2>/dev/null | true

test: build-staking
	mkdir -p tests/wasm
	cp contracts/target/wasm32-unknown-unknown/release/staking_contract.wasm tests/wasm
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
