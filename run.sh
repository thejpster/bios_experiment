#!/bin/bash

set -euo pipefail

cargo build ---manifest-path operatingsystem/Cargo.toml
cp ./operatingsystem/target/debug/liboperatingsystem.so .
cargo run --manifest-path std_bios/Cargo.toml
