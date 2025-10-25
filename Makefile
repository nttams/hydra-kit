# Makefile

# Default target
all: build

build:
	cargo build

build-all:
	cargo build --all-targets

build-tests:
	cargo test --no-run

build-examples:
	cargo build --examples
