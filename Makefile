all: build

build:
	cargo build --release
	strip target/release/yapb

clean:
	cargo clean
