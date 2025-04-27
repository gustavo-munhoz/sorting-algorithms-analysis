#!/usr/bin/env bash

set -euo pipefail

SCRIPT="scripts.run_benchmark"

if [ "$#" -gt 0 ]; then
	python3 -m "$SCRIPT" "$@"
else
	python -m "$SCRIPT" -s 10 100 1000 10000 100000 1000000 10000000
fi
