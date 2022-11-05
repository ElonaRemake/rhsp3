#!/bin/bash

ROOT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/.."
cd "$ROOT_DIR" || exit 1

# Refresh build directory
rm -rf build
mkdir -p build || exit 1
mkdir -p build/examples/x86-windows || exit 1
mkdir -p build/examples/x86-linux || exit 1

# Build exrand cdylib
cargo build -p exrand_hpi --example exrand_hpi_cdylib --release --target i686-pc-windows-gnu || exit 1
cargo build -p exrand_hpi --example exrand_hpi_cdylib --release --target i686-unknown-linux-gnu || exit 1
cargo build -p exrand_hpi --example exrand_hpi_as || exit 1

target/debug/examples/exrand_hpi_as > build/examples/x86-windows/exrand.as
cp target/i686-pc-windows-gnu/release/examples/exrand_hpi_cdylib.dll build/examples/x86-windows/exrand.hpi
cp examples/exrand_hpi/examples/*.hsp build/examples/x86-windows/

target/debug/examples/exrand_hpi_as > build/examples/x86-linux/exrand.as
cp target/i686-unknown-linux-gnu/release/examples/libexrand_hpi_cdylib.so build/examples/x86-linux/exrand.hpi
cp examples/exrand_hpi/examples/*.hsp build/examples/x86-linux/
