#!/usr/bin/env bash

rustup default 1.6.1
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly-2022-07-10-x86_64-unknown-linux-gnu
