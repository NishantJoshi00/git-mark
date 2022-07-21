build:
	cargo build --release
	cargo test --release
install:
	cargo install --path .
uninstall:
	cargo uninstall 