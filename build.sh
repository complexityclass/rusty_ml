#!/bin/bash

# Unset CONDA_PREFIX
unset CONDA_PREFIX

# Activate the virtual environment
source pyground/venv/bin/activate

# Navigate to the Rust project directory
cd rs_ml

# Build the Rust project with Maturin
maturin develop

# Check if maturin develop was successful
if [ $? -ne 0 ]; then
    echo "💥 maturin failed"
    echo "Failed to build Rust project with Maturin"
    exit 1
fi

# Navigate to the pyground directory
cd ../pyground

# Run a specific Python test from the tests directory
# Make sure to replace 'knn_test' with the actual test script name, excluding '.py'
python3 -m tests."$1"

