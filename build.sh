#!/bin/bash

set -e

# Ensure the template binary is built first so the main program can embed it
cargo build -p template

cargo build
