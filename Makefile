.PHONY: all build clean run-server run-client

all: build

build:
	cargo build

clean:
	cargo clean

run-server: build
	cargo run -- server

run-client: build
	cargo run -- client
