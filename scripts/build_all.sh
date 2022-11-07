#!/bin/bash

ROOT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/.."
cd "$ROOT_DIR" || exit 1

# Refresh build directory
rm -rf build
mkdir -p build || exit 1
mkdir -p build/x86-windows/examples || exit 1
mkdir -p build/x86-linux/examples || exit 1

# Build exrand cdylib
cargo build -p exrand_hpi --example exrand_hpi_cdylib --release --target i686-pc-windows-gnu || exit 1
cargo build -p exrand_hpi --example exrand_hpi_cdylib --release --target i686-unknown-linux-gnu || exit 1
cargo build -p exrand_hpi --example exrand_hpi_as || exit 1

target/debug/examples/exrand_hpi_as > build/x86-windows/examples/exrand.as
cp target/i686-pc-windows-gnu/release/examples/exrand_hpi_cdylib.dll build/x86-windows/examples/exrand.hpi
cp examples/exrand_hpi/examples/*.hsp build/x86-windows/examples/

target/debug/examples/exrand_hpi_as > build/x86-linux/examples/exrand.as
cp target/i686-unknown-linux-gnu/release/examples/libexrand_hpi_cdylib.so build/x86-linux/examples/exrand.hpi
cp examples/exrand_hpi/examples/*.hsp build/x86-linux/examples/
