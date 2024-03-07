# recipe-tools

## What is this?

This is a repo to hold useful mini-tools and utilities for working with our structured recipe data (and an excuse to do some practical Rust).

## How do I build/run it?

1. Install the Rust toolchain for your platform
2. Change to the root directory of this repo
3. To run, execute `cargo run --bin find-empty-contributors -- --host recipes.guardianapis.com`.  Since this is designed to hold
multiple tools, you need to tell Cargo which binary to run.  `--` is used as an argument separator; `--host ...` is sent to the _app_ rather than Cargo.
4. To build for your local platform, execute `cargo build`.  You'll find the compiled binaries under `target/debug`.
5. To make a "release" build (with optimizations etc.), execute `cargo build --release`. You'll find the compiled binaries under `target/release`.

## What's in the box?

### find-empty-contributors

Scans the published recipes to find ones that have both `contributors` and `byline` empty or absent.  Displays the relevant Composer path,
CAPI path, uid and checksum (if present).


### recipes-lib

Common library that holds the data models and shared code for interfacing the backend