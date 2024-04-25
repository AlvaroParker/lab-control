#!/bin/bash

run=false
pack=false

# Function to display script usage
function usage() {
  echo "Usage: ./install.sh [--run | --pack]"
  echo "  --run   Run the project."
  echo "  --pack  Pack the project."
}

# Parse command-line arguments
while getopts ":r-:-:" opt; do
  case $opt in
    r)
      run=true
      ;;
    -)
      case "${OPTARG}" in
        run)
          run=true
          ;;
        pack)
          pack=true
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

# Check for exclusivity
if [ "$run" = true ] && [ "$pack" = true ]; then
  echo "Error: Cannot provide both --run and --pack options simultaneously."
  usage
  exit 1
fi

npm --prefix frontend/lab-control install
npm --prefix frontend/lab-control run build

# Build the frontend and copy files to deploy/ folder
yarn --cwd frontend add ./lab-control
yarn --cwd frontend install
yarn --cwd frontend run build
rm -rf deploy/frontend && cp -r frontend/dist deploy/frontend

# Build backend and copy files to deploy/ folder
# cc and openssl and openssl-devel (fedora) or libssl-dev (ubuntu) is needed for this step to work
cargo build --release --manifest-path=backend/Cargo.toml --target-dir=.tmp/
cp ./.tmp/release/backend deploy/backend
cp ./backend/src/database/init.sql deploy/init.sql

# Execute the last line if the "--run" option is provided
if [ "$run" = true ]; then
  cd deploy && docker compose up
fi

if [ "$pack" = true ]; then
  zip -r deploy.zip deploy
fi
