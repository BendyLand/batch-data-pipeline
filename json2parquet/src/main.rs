use clap::Parser;
use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

/// Simple JSON → Parquet converter
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input JSON file (can be NDJSON or array-style)
    input: PathBuf,

    /// Output Parquet file
    output: PathBuf,
}

fn main() -> PolarsResult<()> {
    let args = Args::parse();

    println!("Reading JSON from: {:?}", args.input);
    let input_file = File::open(&args.input)?;
    let mut df = JsonReader::new(input_file)
        .infer_schema_len(Some(1000))
        .with_json_format(JsonFormat::JsonLines) // Use JsonFormat::Json for array-style
        .finish()?;

    println!("Writing Parquet to: {:?}", args.output);
    let output_file = File::create(&args.output)?;
    ParquetWriter::new(output_file)
        .with_compression(ParquetCompression::Zstd(Some(ZstdLevel::try_new(3)?))) 
        .finish(&mut df)?;

    println!("Conversion complete!");
    Ok(())
}

