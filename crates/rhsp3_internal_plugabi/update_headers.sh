#!/bin/bash

bindgen \
  --generate types \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-ord \
  --impl-debug \
  --generate-inline-functions \
  --no-layout-tests \
  openhsp_headers/hsp3struct.h -o src/hsp3struct.rs