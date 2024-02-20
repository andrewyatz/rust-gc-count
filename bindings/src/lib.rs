use pyo3::prelude::*;

pub mod checksumseq;
pub mod models;

use crate::checksumseq::checksum;

/// A Python module implemented in Rust.
#[pymodule]
fn gc_count(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(checksum, m)?)?;
    Ok(())
}