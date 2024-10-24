#!/bin/bash

set -e

folder_path="target/wasm32-unknown-unknown/release"

# Loop through all .wasm files in the folder
for file in "$folder_path"/*.wasm; do
  # Check if the file exists (in case no .wasm files are found)
  if [[ -f "$file" ]]; then
    file_name=$(basename "$file")
    echo "Optimizing $file_name..."

    ./.wasm-opt/binaryen-version_119/bin/wasm-opt -Oz "$file" -o "$file"
    # You can add your specific commands here
  else
    echo "No .wasm files found in the directory."
    break
  fi
done
echo "Done."