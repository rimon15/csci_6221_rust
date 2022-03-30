#!/usr/bin/env bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
sudo apt install pkg-config libssl-dev
cargo install cargo-generate
npm install npm@latest -g