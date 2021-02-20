#!/bin/bash

set -euo pipefail

echo "Trying with Rust OS"
cargo build ---manifest-path operatingsystem/Cargo.toml
cp ./operatingsystem/target/debug/liboperatingsystem.so .
cargo run --manifest-path std_bios/Cargo.toml

echo "Trying with C OS"
cbindgen -o operatingsystem_c/bios_common.h bios_common
gcc -shared -o liboperatingsystem.so -I operatingsystem_c operatingsystem_c/lib.c
cargo run --manifest-path std_bios/Cargo.toml

