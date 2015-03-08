all:
	cargo build

test:
	cargo test

clean:
	cargo clean

release:
	cargo build --release

bench:
	cargo bench
