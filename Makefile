build:
	cargo build

test:
	cargo test

test_overwrite:
	REGENERATE_GOLDENFILES=1 TRYBUILD=overwrite cargo test

publish: build test
	cargo publish -p async-debug-derive
	sleep 15
	cargo publish -p async-debug
