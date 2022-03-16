#!/bin/bash

cd $(dirname $0)

cd ./client

./build-wasm.sh

cd ../server-wasmedge

OUTPUT_CSS="$(pwd)/../client/build/app.css" cargo run -p isomorphic-server-wasmedge
