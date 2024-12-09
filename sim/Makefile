.DEFAULT_GOAL := run

################################################
#### RUST BUILD AND RUN COMMANDS
################################################

## run: Build (if files changed) and run cargo project
.PHONY: run
run: 
	cargo run

## run-rel: Run the release executable
.PHONY: run-rel
run-rel:
	cargo run -r

## check: Check to see code compiles (does not produce executable)
.PHONY: check
check: 
	cargo check

## build-rel: Build executable in release mode (optimized)
.PHONY: build-rel
build-rel: 
	cargo build --release

## build: Build executable in debug mode
.PHONY: build
build: 
	cargo build

################################################
#### RUST CODE TOOLS
################################################

## format: Format rust code
.PHONY: format
format: 
	cargo fmt

## fix: Automatically fix compiler warnings and manage rust editions
.PHONY: fix
fix: 
	cargo fix

## lint: Lint rust code
.PHONY: lint
lint:
	cargo clippy

## docs: Generate docs for all dependencies and open in the browser.
.PHONY: docs
docs:
	cargo doc --open

################################################
#### RUST CARGO PROJECT COMMANDS
################################################

## update: Update cargo crates. Ignores current cargo.lock and then generates a new one when done.
.PHONY: update
update:
	cargo update