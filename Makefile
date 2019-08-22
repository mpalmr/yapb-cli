all: clean build

build:
	cargo build --release
	strip target/release/yapb

dev:
	cargo build

clean:
	cargo clean

test-new: dev
	target/debug/yapb new rustfmt.toml
