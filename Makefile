build:
	cargo build

test:
	cargo test

publish: build test
	$(MAKE) -C crates/async-debug-derive publish
	sleep 15
	cargo publish
