#!/bin/bash

cd ../../generator

if [[ -n $1 ]]; then
  ./generator $1 &
else
  ./generator &
fi
RUST_PS=$!

cd ../processor/scripts

echo "Generating data..."
wait "$RUST_PS"
echo "Done!"

