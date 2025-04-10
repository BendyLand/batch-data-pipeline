#!/bin/bash

source processor/env/bin/activate

cd processor

./cycle.sh 5 200000

python3 process.py

echo "SELECT * FROM orders LIMIT 10;" | pbcopy
duckdb orders.duckdb

./reset_files.sh
cd ..

source ~/.bashrc

