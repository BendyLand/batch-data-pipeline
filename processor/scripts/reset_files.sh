#!/bin/bash

cd ..
echo "Removing all 'data*.parquet*' files..."
rm -f data*.parquet*
echo "Removing 'orders.duckdb'..."
rm -f orders.duckdb
echo "Done!"
cd scripts

