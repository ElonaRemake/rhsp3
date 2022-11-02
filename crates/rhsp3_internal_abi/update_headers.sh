#!/bin/bash

do_bindgen() {
  bindgen \
    --generate types,vars \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-ord \
    --impl-partialeq \
    --impl-debug \
    --generate-inline-functions \
    --no-layout-tests \
    --newtype-global-enum ".*" \
    "$@" -- \
    -fexceptions -target i686-pc-windows-msvc
}
fix_abi() {
  sed -i "s/extern \"C\"/extern \"C-unwind\"/g" "$@"
}

do_bindgen \
  --blocklist-item hspvarproc \
  --blocklist-item hspvartype_max \
  --blocklist-item mem_pval \
  openhsp_headers/hsp3struct.h -o src/hsp3struct.rs
fix_abi src/hsp3struct.rs
