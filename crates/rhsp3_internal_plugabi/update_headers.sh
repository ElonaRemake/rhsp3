#!/bin/bash

bindgen \
  --generate types \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-ord \
  --impl-debug \
  --generate-inline-functions \
  openhsp_headers/hsp3struct.h -o src/hsp3struct.rs -- -target i686-pc-windows-gnu