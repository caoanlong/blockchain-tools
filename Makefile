.PHONY: build

build:
	cargo build --release

exec:
	./target/release/blockchain-tools