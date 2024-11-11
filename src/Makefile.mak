
build:
	cargo build

release:
	cargo build --release

run:
	cargo run

run_parse_example:
	cargo run parse 

test:
	cargo test

clippy:
	cargo clippy 

format:
	cargo fmt
	
pcommit: format clippy test