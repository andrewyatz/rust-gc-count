use pyo3::prelude::*;
use rust_gc_count::checksum::ChecksumResult;

#[pyclass]
#[pyo3(name="ChecksumResult")]
pub struct PyChecksumResult {
    #[pyo3(get,set)]
    pub id: String,
    #[pyo3(get,set)]
    pub length: usize,
    #[pyo3(get,set)]
    pub sha512: String,
    #[pyo3(get,set)]
    pub md5: String
}

#[pymethods]
impl PyChecksumResult {
    fn __repr__(&self) -> String {
        format!("<ChecksumResult for {}>", self.id)
    }
}

impl From<ChecksumResult> for PyChecksumResult {
    fn from(value: ChecksumResult) -> Self {
        PyChecksumResult {
            id: value.id,
            length: value.length,
            sha512: value.sha512,
            md5: value.md5
        }
    }
}