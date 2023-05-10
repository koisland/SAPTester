#!/usr/bin/env bash

# Deploy backend database.
cd backend/
fly deploy

# Compile frontend code to WASM and move to 'docs/'. Github CI for Pages does the rest on commit.
cd ../frontend/
trunk build --release --public-url SAPTester
rm -rf ../docs && mkdir -p ../docs
mv dist/* ../docs
