#!/bin/bash
wasm-pack build
pushd .
cd www
npm start
popd
