build:
	cargo build

test:
	cargo test

publish: build test
	cargo publish -p async-debug-derive
	sleep 15
	cargo publish -p async-debug
