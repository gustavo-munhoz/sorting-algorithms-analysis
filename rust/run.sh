#!/usr/bin/env bash

set -euo pipefail

echo "Running tests..."

# Roda os testes
if cargo test -- --test-threads=1 --nocapture; then
    echo ""
    echo "✅ All tests passed."
else
    echo ""
    echo "❌ Some tests failed. Check the errors above."
    exit 1
fi

echo ""
echo "🏁 Running benchmark..."

# Rodar o benchmark agora
if [ "$#" -gt 0 ]; then
    cargo run --release -- "$@"
else
    cargo run --release -- -s 10,100,1000,10000,100000,1000000,10000000
fi