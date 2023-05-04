#!/usr/bin/env bash

cargo watch -d 2 -w src -x 'run --bin server -- --port 8081'
