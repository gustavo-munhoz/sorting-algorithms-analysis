#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -gt 0 ]; then
	cargo run --release -- "$@"
else
	cargo run --release -- -s 10,100,1000,10000,100000,1000000,10000000
fi
