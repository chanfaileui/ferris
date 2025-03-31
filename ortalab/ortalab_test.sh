#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Initialize counters
total_tests=0
passed_tests=0
failed_tests=0

# Function to run a single test
run_test() {
    local test_file="$1"
    local output_file="${test_file%.yml}.output"

    # Skip if output file doesn't exist
    if [ ! -f "$output_file" ]; then
        echo -e "${YELLOW}Skipping $test_file - No output file found${NC}"
        return
    fi

    echo -e "Testing: ${YELLOW}$test_file${NC}"
    total_tests=$((total_tests + 1))

    # # Run the program and capture output
    # actual_output=$(cargo run --quiet -- "$test_file" --explain)

    # Run the program and capture output, redirecting stderr to /dev/null to ignore warnings
    actual_output=$(cargo run --quiet -- "$test_file" --explain 2>/dev/null)

    expected_output=$(cat "$output_file")

    # Compare output (ignoring whitespace differences)
    if [ "$(echo "$actual_output" | tr -d '[:space:]')" = "$(echo "$expected_output" | tr -d '[:space:]')" ]; then
        echo -e "${GREEN}✓ PASSED${NC}"
        passed_tests=$((passed_tests + 1))
    else
        echo -e "${RED}✗ FAILED${NC}"
        echo -e "${YELLOW}Expected:${NC}"
        echo "$expected_output"
        echo -e "${YELLOW}Actual:${NC}"
        echo "$actual_output"
        failed_tests=$((failed_tests + 1))
    fi
}

# Recursive function to find and test all yml files
test_directory() {
    local dir="$1"

    # Process all yml files in this directory
    for test_file in "$dir"/*.yml; do
        if [ -f "$test_file" ]; then
            run_test "$test_file"
        fi
    done

    # Recursively process subdirectories
    for subdir in "$dir"/*/; do
        if [ -d "$subdir" ]; then
            test_directory "$subdir"
        fi
    done
}

# Main function
main() {
    # runs all categorised tests
    # or, specify a different directory to run tests from like
    # usage: ./test_ortalab.sh fixtures/categorised/enhancements
    # can expand to own personal tests
    local test_dir="${1:-fixtures/categorised}"

    echo "===== Starting Ortalab Tests ====="
    echo "Test directory: $test_dir"

    # Run all tests
    test_directory "$test_dir"

    # Summary
    echo "===== Test Summary ====="
    echo -e "Total tests: $total_tests"
    echo -e "${GREEN}Passed: $passed_tests${NC}"
    if [ $failed_tests -gt 0 ]; then
        echo -e "${RED}Failed: $failed_tests${NC}"
        exit 1
    else
        echo -e "All tests passed! 🎉"
        exit 0
    fi
}

# Run the main function with the provided directory or default
main "$1"