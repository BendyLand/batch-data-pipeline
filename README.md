# Local Batch Pipeline & Data Generator

A self-contained local data engineering pipeline that:

 1) **Generates** millions of synthetic order records in Rust (Parquet format).  
 2) **Ingests** and **normalizes** them into DuckDB tables via Python.  
 3) **Processes** the normalized data to compute key metrics and analytics.  
 4) **Runs** sample SQL queries to explore the results.

---

## Features

 - **High-volume data generation**  
    - Default: 2 000 000 orders generated in parallel using Rust & `rayon`  
 - **Parquet output** for efficient columnar storage  
 - **DuckDB** for ultra-fast local OLAP  
 - **Python scripts** for table creation, normalization, and analytics  
 - **Bash orchestration** for end-to-end automation, timing, and cleanup  
 - **Sample queries** in `sample.sql` for quick insights  

---

## Tech Stack

 - **Generator:** Rust -> Parquet via `arrow2`  
 - **Orchestration:** Bash (`demo.sh`, `cycle.sh`, etc.)  
 - **Storage & Analytics:** DuckDB + Python (`duckdb` package)  
 - **UUIDs:** `uuidgen` for unique filenames  

---

## Prerequisites

 - **MacOS** (or Linux)  
 - **Rust toolchain** (Cargo)  
 - **Python 3.8+**  
 - **DuckDB Python package:**  
```bash
pip install duckdb
```
 - uuidgen, bash, make (optional)

## Setup

 1) Clone and enter repo.
```bash
git clone https://github.com/BendyLand/batch-data-pipeline
cd batch-data-pipeline
```

 2) Build the Rust generator.
```bash
cd generator
cargo build --release
cp target/release/json2parquet ./generator
cd ..
```

 3) Create & Python venv and install dependencies.
```bash
python3 -m venv env
source env/bin/activate
pip install duckdb
```
 
 4) Ensure scripts are executable.
```bash
chmod +x $(find . -name "*.sh" | paste -sd\  -)
```

## Usage

From the project root, simply run:
```bash
./demo.sh
```
 - Default behavior
     - 5 iterations × 200 000 orders
     - Full pipeline (gen -> ingest -> normalize -> process -> sample queries)
     - Custom iterations & batch size
 - Custom iterations & batch size
     - `./processor/scripts/cycle.sh 10 500000 # 10 batches of 500_000`

## Script Details

 - demo.sh
       - Changes into processor/scripts
       - Runs cycle.sh (gen -> ingest -> normalize -> move parquet)
       - Cleans up via reset_files.sh
       - Executes process.py and times the overall run
       - Finally, runs duckdb orders.duckdb < sample.sql

 - cycle.sh [ITERATIONS] [COUNT]
       - Toggles venv (toggle_env.sh)
       - Loops ITERATIONS times (default 5)
          - Generate COUNT orders (default 2 000 000) via gen_data.sh
          - Ingest & normalize to DuckDB
          - Rename data.parquet -> data-<UUID>.parquet
       - After looping, runs full analytics via process.py

 - gen_data.sh [COUNT]
       - Invokes Rust generator [COUNT] in background
       - Waits for it, producing processor/data.parquet

 - reset_files.sh
       - Deletes all data*.parquet*
       - Deletes orders.duckdb

 - toggle_env.sh
       - If venv active -> deactivate
       - Else -> activate env/bin/activate

⸻

## Python Pipeline
1) ingest.py
   - Creates table incoming_orders from data.parquet
2) normalize.py
   - customers, products, orders tables
   - Populates them from incoming_orders
3) process.py
   - order_anomalies (canceled/refunded + “BadId” checks)
   - top_product, monthly_order_volume, top_spenders, product_return_rates, customer_anomalies
   - Final .duckdb ready for querying

⸻

## Sample Queries

Run:
```bash
duckdb orders.duckdb < processor/scripts/sample.sql
```

You’ll see:
 1.	Total counts (customers, products, orders)
 2.	Top spenders & products
 3.	Highest return rates
 4.	Monthly order volumes
 5.	Anomaly summaries & recent orders

⸻

## Customization
 - Iterations & batch size
```bash
./processor/scripts/cycle.sh <iterations> <records_per_batch>
```
 - Python venv path
    - Edit VENV_PATH in toggle_env.sh if you move your venv

## License

This project is licensed under the MIT License.

