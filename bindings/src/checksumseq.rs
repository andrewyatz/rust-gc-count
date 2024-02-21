use pyo3::prelude::*;

use seq_io::fasta::Reader;
use rust_gc_count::checksum::process_sequence;
use crate::models::PyChecksumResult;

#[pyfunction]
pub fn checksum(file: String, verbose: Option<bool>) -> Vec<PyChecksumResult> {
    
    let mut results = Vec::new();
    let mut reader = Reader::from_path(file).unwrap();

    let verbose = verbose.unwrap_or(false);

    while let Some(record) = reader.next() {
        let record = record.unwrap();
        let result = process_sequence(record, verbose);

        results.push(PyChecksumResult::from(result));
    }

    results
}