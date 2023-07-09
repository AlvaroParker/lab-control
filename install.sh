#!/bin/bash

run=false

# Function to display script usage
function usage() {
  echo "Usage: ./script.sh [--run]"
}

# Parse command-line arguments
while getopts ":r-:" opt; do
  case $opt in
    r)
      run=true
      ;;
    -)
      case "${OPTARG}" in
        run)
          run=true
          ;;
        *)
          usage
          exit 1
          ;;
      esac
      ;;
    *)
      usage
      exit 1
      ;;
  esac
done

# Build the frontend and copy files to deploy/ folder
yarn --cwd frontend run build
rm -rf deploy/frontend && cp -r frontend/dist deploy/frontend

# Build backend and copy files to deploy/ folder
cargo build --manifest-path backend/Cargo.toml --release
cp ~/.cargo-target/release/backend deploy/backend
cp ./backend/src/database/init.sql deploy/init.sql

# Execute the last line if the "--run" option is provided
if [ "$run" = true ]; then
  cd deploy && docker compose up
fi
