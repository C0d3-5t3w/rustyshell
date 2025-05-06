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

help:
	@echo "Makefile for building and running the project"
	@echo ""
	@echo "Usage:"
	@echo "  make all          Build the project"
	@echo "  make build        Build the project"
	@echo "  make clean        Clean the project"
	@echo "  make run-server   Run the server"
	@echo "  make run-client   Run the client"
	@echo "  make help         Show this help message"
