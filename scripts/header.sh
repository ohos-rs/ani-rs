#!/bin/bash

CURRENT_DIR=$(dirname "$0")

bindgen \
$CURRENT_DIR/../include/ani.h \
-o $CURRENT_DIR/../crates/sys/src/lib.rs \
--allowlist-function 'ani.*' \
--allowlist-var 'ani.*' \
--allowlist-type 'ani.*' \
--raw-line '#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]'