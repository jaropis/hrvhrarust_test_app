// in their main.rs or lib.rs
use hrvhra_rust::{data_reader::RRSeries, runs::RRRuns};

fn main() {
    // Method 1: Direct vector input
    let rr_data = vec![1.0, 2.0, 3.0, 2.0, 1.0];
    let annot_data = vec![0, 0, 0, 0, 0];
    let mut rr = RRRuns::new(rr_data, annot_data, true);
    rr.get_full_runs();
    let summary = rr.get_runs_summary();
    println!("Analysis results: {:?}", summary);

    // Method 2: Reading from file
    match RRSeries::read_rr("test1.csv") {
        Ok(rr_series) => {
            let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
            rr.get_full_runs();
            let summary = rr.get_runs_summary();
            println!("File analysis results: {:?}", summary);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
