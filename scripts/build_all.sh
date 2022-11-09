#!/bin/bash

ROOT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/.."
cd "$ROOT_DIR" || exit 1

build_example_hpi() {
  cargo build -p "$1"_hpi --example "$1"_hpi_cdylib --release --target i686-pc-windows-gnu || exit 1
  cargo build -p "$1"_hpi --example "$1"_hpi_cdylib --release --target i686-unknown-linux-gnu || exit 1
  cargo build -p "$1"_hpi --example "$1"_hpi_as || exit 1

  target/debug/examples/"$1"_hpi_as > build/x86-windows/examples/"$1"/"$1".as
  cp target/i686-pc-windows-gnu/release/examples/"$1"_hpi_cdylib.dll build/x86-windows/examples/"$1"/"$1".hpi

  target/debug/examples/"$1"_hpi_as > build/x86-linux/examples/"$1"/"$1".as
  cp target/i686-unknown-linux-gnu/release/examples/lib"$1"_hpi_cdylib.so build/x86-linux/examples/"$1"/"$1".hpi
}

# Refresh build directory
rm -rf build
mkdir -p build || exit 1
mkdir -p build/x86-windows || exit 1
mkdir -p build/x86-linux || exit 1

# Create examples directory
cp -r data/* build/x86-windows || exit 1
cp -r data/* build/x86-linux || exit 1
build_example_hpi exrand
build_example_hpi z
