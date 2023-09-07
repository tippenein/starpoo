all: build run

build:
	cargo build

run:
	cargo run
watch:
	cargo watch -x run
