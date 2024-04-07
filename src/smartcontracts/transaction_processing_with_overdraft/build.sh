#!/bin/bash

cargo +nightly-2023-06-25 build --target wasm32-unknown-unknown --release -Zbuild-std -Zbuild-std-features=panic_immediate_abort
cp ./target/wasm32-unknown-unknown/release/transacАtion_processing_with_overdraft.wasm ../../resources/transaction_processing_with_overdraft.wasm