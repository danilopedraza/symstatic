#!/bin/bash

set -e

cargo test --all-features

set +e
examples_dir=$1
echo "Building the interpreter..."
cargo build

echo "Running examples..."
someone_failed=false
excluded_files=("sort-types.komodo")
for file in $(find "$examples_dir" -type f -name "*.komodo"); do
    filename=$(basename "$file")
    excluded=false
    for excluded_file in "${excluded_files[@]}"; do
        if [ "$filename" = "$excluded_file" ]; then
            excluded=true
            break
        fi
    done
    if [ "$excluded" = true ]; then
        echo "⏭️  Skipping excluded file: $file"
        continue
    fi

    output=$(eval "cargo run --quiet $file" 2>&1)

    if [ $? -ne 0 ]; then
        echo "❌ Error: $file failed its execution" >&2
        echo "This is the message from the interpreter:" >&2
        echo "$output" >&2
        someone_failed=true
    fi
done

if [ "$someone_failed" = true ] ; then
    exit 1
fi

echo "✅ All the example files were executed successfully!"
echo "All the tests passed!"
