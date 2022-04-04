#!/usr/bin/env bash

wasm-pack build
fuser -k 8080/tcp
cd www/ && npm run start