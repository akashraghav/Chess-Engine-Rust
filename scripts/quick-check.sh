#!/bin/bash
set -e

# Quick CI check script - runs the most essential checks
# Usage: ./scripts/quick-check.sh

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[CI]${NC} $1"; }
print_success() { echo -e "${GREEN}[✓]${NC} $1"; }
print_error() { echo -e "${RED}[✗]${NC} $1"; }

echo "⚡ Quick CI Check"
echo "=================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in project root directory"
    exit 1
fi

FAILED=0

# Essential checks
print_status "Checking code formatting..."
if ! cargo fmt --all -- --check; then
    print_error "Code formatting failed"
    FAILED=1
else
    print_success "Code formatting"
fi

print_status "Running clippy on core components..."
if ! cargo clippy -p chess-core -p chess-engine --all-targets --all-features -- -D warnings; then
    print_error "Clippy failed"
    FAILED=1
else
    print_success "Clippy lints"
fi

print_status "Building project..."
if ! cargo build; then
    print_error "Build failed"
    FAILED=1
else
    print_success "Build"
fi

print_status "Running core tests..."
if ! cargo test --lib -p chess-core -p chess-engine; then
    print_error "Tests failed"
    FAILED=1
else
    print_success "Core tests"
fi

echo ""
if [ $FAILED -eq 0 ]; then
    print_success "All essential checks passed! 🎉"
    echo "Ready for detailed CI with: ./scripts/run-ci-locally.sh"
else
    print_error "Some checks failed. Fix issues before running full CI."
    exit 1
fi