#!/bin/bash

CURRENT_DIR=$(dirname "$0")

bindgen \
$CURRENT_DIR/../include/ani.h \
-o $CURRENT_DIR/../crates/sys/src/lib.rs \
--allowlist-function 'ani.*' \
--allowlist-function 'ANI.*' \
--allowlist-var 'ani.*' \
--allowlist-var 'ANI.*' \
--allowlist-type 'ani.*' \
--allowlist-type 'ANI.*' \
--raw-line '#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]'