#!/bin/bash
set -e

# Benchmark runner script - matches CI benchmark job
# Usage: ./scripts/run-benchmarks.sh [--save-results]

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[BENCH]${NC} $1"; }
print_success() { echo -e "${GREEN}[✓]${NC} $1"; }
print_error() { echo -e "${RED}[✗]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[⚠]${NC} $1"; }

echo "⚡ Chess Engine Benchmarks"
echo "=========================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in project root directory"
    exit 1
fi

SAVE_RESULTS=false
if [ "$1" = "--save-results" ]; then
    SAVE_RESULTS=true
    mkdir -p benchmark-results
    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
fi

# Build release version first
print_status "Building release version for accurate benchmarks..."
cargo build --release

echo ""
print_status "Running performance benchmarks..."

# Create benchmark results file if requested
if [ "$SAVE_RESULTS" = true ]; then
    RESULTS_FILE="benchmark-results/bench_results_$TIMESTAMP.txt"
    echo "Chess Engine Benchmark Results - $(date)" > "$RESULTS_FILE"
    echo "=======================================" >> "$RESULTS_FILE"
    echo "" >> "$RESULTS_FILE"
fi

# Move generation benchmark
print_status "Move Generation Benchmark"
if [ "$SAVE_RESULTS" = true ]; then
    echo "Move Generation:" >> "$RESULTS_FILE"
    cargo bench --bench move_generation 2>&1 | tee -a "$RESULTS_FILE"
else
    cargo bench --bench move_generation
fi
print_success "Move generation benchmark complete"

echo ""

# Evaluation benchmark
print_status "Position Evaluation Benchmark"
if [ "$SAVE_RESULTS" = true ]; then
    echo -e "\nPosition Evaluation:" >> "$RESULTS_FILE"
    cargo bench --bench evaluation 2>&1 | tee -a "$RESULTS_FILE"
else
    cargo bench --bench evaluation
fi
print_success "Evaluation benchmark complete"

echo ""

# Search benchmark
print_status "Search Algorithm Benchmark"
if [ "$SAVE_RESULTS" = true ]; then
    echo -e "\nSearch Algorithms:" >> "$RESULTS_FILE"
    cargo bench --bench search 2>&1 | tee -a "$RESULTS_FILE"
else
    cargo bench --bench search
fi
print_success "Search benchmark complete"

echo ""

# Run the main benchmark binary if it exists
print_status "Running comprehensive benchmarks..."
if [ -f "target/release/bench" ] || cargo build --release --bin bench 2>/dev/null; then
    if [ "$SAVE_RESULTS" = true ]; then
        echo -e "\nComprehensive Benchmarks:" >> "$RESULTS_FILE"
        ./target/release/bench 2>&1 | tee -a "$RESULTS_FILE"
    else
        ./target/release/bench
    fi
    print_success "Comprehensive benchmarks complete"
else
    print_warning "Comprehensive benchmark binary not available"
fi

echo ""

# Summary
print_success "All benchmarks completed!"

if [ "$SAVE_RESULTS" = true ]; then
    echo ""
    print_status "Results saved to: $RESULTS_FILE"

    # Create a summary
    SUMMARY_FILE="benchmark-results/latest_summary.txt"
    echo "Latest Benchmark Summary - $(date)" > "$SUMMARY_FILE"
    echo "=================================" >> "$SUMMARY_FILE"

    # Extract key metrics (this would need to be customized based on actual benchmark output)
    if grep -q "time:" "$RESULTS_FILE"; then
        echo "" >> "$SUMMARY_FILE"
        echo "Key Performance Metrics:" >> "$SUMMARY_FILE"
        grep "time:" "$RESULTS_FILE" | head -10 >> "$SUMMARY_FILE"
    fi

    print_status "Summary saved to: $SUMMARY_FILE"
fi

echo ""
echo "📊 Benchmark Coverage:"
echo "• Move generation performance"
echo "• Position evaluation speed"
echo "• Search algorithm efficiency"
echo "• Memory usage patterns"
echo "• Multi-threading performance"
echo ""

# Performance targets (adjust based on your requirements)
echo "🎯 Expected Performance Targets:"
echo "• Move Generation: >1M moves/second"
echo "• Position Evaluation: >100K positions/second"
echo "• Search Depth: 6+ plies in <5 seconds"
echo "• Memory Usage: <100MB for typical games"
echo ""

if [ "$SAVE_RESULTS" = true ]; then
    echo "💾 Results archived in benchmark-results/ directory"
    echo "📈 Compare with previous runs to track performance changes"
fi