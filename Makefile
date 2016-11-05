all: build

build:
	@cargo build

doc:
	@cargo doc

test:
	@cargo test


.PHONY: all build doc test
