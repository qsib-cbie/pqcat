//!
//! This binary takes one argument, the path to a parquet file.
//!
//! It opens the parquet file using Polars, then prints the debug representation of the DataFrame.
//!
//! # Example
//! ```shell
//! pqcat data.parquet
//!
//! # Output
//! shape: (9_720_794, 4)
//! ┌────────────────────────────┬───────┬───────┬──────┐
//! │ t                          ┆ x     ┆ y     ┆ z    │
//! │ ---                        ┆ ---   ┆ ---   ┆ ---  │
//! │ datetime[μs]               ┆ i32   ┆ i32   ┆ i32  │
//! ╞════════════════════════════╪═══════╪═══════╪══════╡
//! │ 2023-06-15 20:35:23.734994 ┆ 1713  ┆ 215   ┆ 5783 │
//! │ 2023-06-15 20:35:23.735290 ┆ 1713  ┆ 215   ┆ 7763 │
//! │ 2023-06-15 20:35:23.735586 ┆ 1713  ┆ 215   ┆ 7673 │
//! │ 2023-06-15 20:35:23.735882 ┆ 1713  ┆ 215   ┆ 7762 │
//! │ 2023-06-15 20:35:23.736178 ┆ 1713  ┆ 215   ┆ 7673 │
//! │ …                          ┆ …     ┆ …     ┆ …    │
//! │ 2023-06-15 21:23:40.543990 ┆ -1126 ┆ -4599 ┆ 6770 │
//! │ 2023-06-15 21:23:40.544286 ┆ -1126 ┆ -4599 ┆ 6758 │
//! │ 2023-06-15 21:23:40.544582 ┆ -1126 ┆ -4599 ┆ 6756 │
//! │ 2023-06-15 21:23:40.544878 ┆ -1126 ┆ -4599 ┆ 6747 │
//! │ 2023-06-15 21:23:40.545174 ┆ -1126 ┆ -4599 ┆ 6741 │
//! └────────────────────────────┴───────┴───────┴──────┘
//! ```

use polars::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <parquet file>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let df = LazyFrame::scan_parquet(path, Default::default()).expect("Valid parquet file").collect().expect("Parquet file to be read");
    println!("{:?}", df);
}
