#!/bin/bash

cargo +nightly-2023-06-25 build --release -Zbuild-std -Zbuild-std-features=panic_immediate_abort --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/transacАtion_processing_with_overdraft.wasm ../../resources/transaction_processing_with_overdraft.wasm