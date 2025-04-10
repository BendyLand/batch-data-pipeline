#!/bin/bash

source env/bin/activate

START_TIME=$(date +%s)

ITERATIONS=${1:-5} # Defaults to 5 if not specified
COUNT=0

while true; do
  if [[ $ITERATIONS -ne 0 && $COUNT -ge $ITERATIONS ]]; then
    echo "Reached $ITERATIONS iterations. Exiting."
    break
  fi

  echo ""
  echo "=== Iteration $((COUNT + 1)) ==="

  if [[ -n $2 ]]; then
    ./gen_data.sh $2
  else 
    ./gen_data.sh
  fi

  if [[ -f "data.parquet" ]]; then
    echo "Ingesting data..."
    python3 ingest.py
    echo "Done!"
    echo "Normalizing data..."
    python3 normalize.py
    echo "Done!"
    UUID=$(uuidgen)
    mv data.parquet "data-$UUID.parquet"
    echo "Data moved to 'data-$UUID.parquet'."
  else
    echo "No data.parquet found. Skipping..."
  fi

  ((COUNT++))
done

echo ""
echo "Processing data..."
python3 process.py

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

# Format elapsed time
if [[ $ELAPSED -ge 60 ]]; then
  MINUTES=$((ELAPSED / 60))
  SECONDS=$((ELAPSED % 60))
  TIME_MSG="${MINUTES}m ${SECONDS}s"
else
  TIME_MSG="${ELAPSED}s"
fi

ENTRIES_PER_ITER=${2:-2000000}

echo ""
echo "Finished $COUNT iteration(s) for $ENTRIES_PER_ITER entries each in $TIME_MSG."
echo ""

echo "Running sample queries..."
duckdb orders.duckdb < sample.sql

source ~/.bashrc

