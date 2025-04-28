# === Stage 1: Build the generator (Rust) ===
# This stage uses a Rust image to compile your generator binary.
FROM rust:1.75 AS builder

# Set the working directory to the generator directory
WORKDIR /app

# Copy your generator code. Adjust as needed if your generator project structure differs.
COPY generator/ ./generator/

WORKDIR /app/generator
# Build the generator in release mode.
RUN cargo build --release 

# === Stage 2: Create the final image ===
# Use a lightweight Python image as the base, which also gives you Python and slim system resources.
FROM python:3.12-slim
WORKDIR /app

# Install any system dependencies (for example, uuid-runtime is used by your shell scripts).
RUN apt-get update && apt-get install -y \
    uuid-runtime \
    curl \
    unzip \
 && rm -rf /var/lib/apt/lists/*

# Install DuckDB CLI (latest stable release)
RUN curl -L -o /usr/local/bin/duckdb.zip https://github.com/duckdb/duckdb/releases/latest/download/duckdb_cli-linux-aarch64.zip \
 && unzip /usr/local/bin/duckdb.zip -d /usr/local/bin \
 && rm /usr/local/bin/duckdb.zip \
 && chmod +x /usr/local/bin/duckdb

# Copy the processor into the container.
COPY processor/ ./processor/
COPY demo.sh ./

# Copy the built generator binary from the builder stage.
# The binary is located in the builder at /app/generator/target/release/generator.
COPY --from=builder /app/generator/target/release/generator /app/generator/generator

# Install Python dependencies. If you have a requirements.txt, use:
# RUN pip install --no-cache-dir -r requirements.txt
# Here we install duckdb as an example.
WORKDIR /app/processor
RUN pip install -r requirements.txt

WORKDIR /app
# Ensure that all your shell scripts in the scripts directory are executable.
RUN chmod +x processor/scripts/*.sh
RUN chmod +x demo.sh

# Set the container's default command to run your demo.sh.
# This script will navigate through directories and run cycle.sh and reset_files.sh as defined.
CMD ["./demo.sh"]
