#!/usr/bin/env bash

set -euo pipefail

TEST_SCRIPT="scripts.run_tests"
BENCHMARK_SCRIPT="scripts.run_benchmark"

# Rodar os testes primeiro
echo "Running tests..."
python3 -m "$TEST_SCRIPT"

# Se chegou aqui, significa que passou
echo "✅ All tests passed. Running benchmark..."

# Rodar benchmark
if [ "$#" -gt 0 ]; then
    python3 -m "$BENCHMARK_SCRIPT" "$@"
else
    python3 -m "$BENCHMARK_SCRIPT" -s 10 100 1000 10000 100000 500000 
fi
