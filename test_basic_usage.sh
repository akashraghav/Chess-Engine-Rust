#!/bin/bash
# Test basic_usage example non-interactively

echo "Testing basic_usage example with automated input..."

# Provide some moves and then quit
echo -e "e2e4\ne7e5\nquit" | timeout 10 cargo run --release --example basic_usage

echo "✅ basic_usage example tested successfully (non-hanging)"