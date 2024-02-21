use pyo3::prelude::*;

pub mod gc_count_utils;
pub mod checksumseq;
pub mod models;

use crate::checksumseq::checksum;
use crate::gc_count_utils::write_gc_count_to_file;

/// A Python module implemented in Rust.
#[pymodule]
fn gc_count(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(checksum, m)?)?;
    m.add_function(wrap_pyfunction!(write_gc_count_to_file, m)?)?;
    Ok(())
}