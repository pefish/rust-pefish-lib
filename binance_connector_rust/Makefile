

DEFAULT: build-release

build-release:
	cargo build --release

build:
	cargo build

publish:
	cargo publish --allow-dirty

test:
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --exact --nocapture