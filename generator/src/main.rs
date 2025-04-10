use std::env;

use rayon::prelude::*;
use crate::orders::generate_order;

mod customers;
mod payments;
mod products;
mod utils;
mod orders;
mod writer;

fn main() {
    // ----------------------- Parse input arg -----------------------
    let args: Vec<String> = env::args().collect();
    let num_orders: usize = if args.len() > 1 {
        args[1].parse().unwrap_or_else(|_| {
            eprintln!("Non-numeric argument passed for number of orders.");
            std::process::exit(1);
        })
    } 
    else { 2_000_000 };

    // ----------------------- Generate in parallel -----------------------
    println!("Generating {num_orders} orders...");
    let orders: Vec<orders::Order> = (0..num_orders)
	    .into_par_iter()
	    .map(|_| generate_order())
	    .collect();

    // ----------------------- Write to file -----------------------
    let path = "../processor/data.parquet";
	if let Err(err) = writer::write_parquet(&orders, path) {
	    eprintln!("Failed to write Parquet file: {err}");
	}
	else {
		println!("Data generated successfully!")
	}
}


