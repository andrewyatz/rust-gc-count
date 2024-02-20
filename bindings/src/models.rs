use pyo3::prelude::*;

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