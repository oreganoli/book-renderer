#!/bin/sh
cargo build --release
mkdir -p release
cp target/release/book-renderer ./release/book-renderer