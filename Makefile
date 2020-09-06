
.PHONY: release, test

release:
	cargo build --release
	# strip target/release/website

build:
	cargo build

test:
	rm -rf ./data
	cargo test
	rm -rf ./data
	mkdir data
