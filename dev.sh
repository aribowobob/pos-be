#!/bin/bash

# Development script with cargo-watch
# This will watch for changes in your src directory and restart the application automatically

# Add any environment variables needed for development
export RUST_LOG=debug

# Run cargo watch to monitor file changes and restart the app
cargo watch -x 'run'

# For more advanced options:
# cargo watch -x check -x test -x run
# This will run check, then tests, then the application
