build:
	cargo build

test:
	cargo test -p

publish: build test
	publish -p async-debug-derive
	sleep 15
	cargo publish -p async-debug
