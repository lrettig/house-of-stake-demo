#!/bin/bash
set -e

cd "$(dirname $0)"
cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/venear_contract.wasm ./res/
cp target/wasm32-unknown-unknown/release/voting_contract.wasm ./res/ 