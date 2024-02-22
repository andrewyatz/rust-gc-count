use pyo3::prelude::*;
use std::path::PathBuf;

#[pyfunction]
pub fn write_gc_count_to_file(
    input: String,
    output: String,
    compression_level: u32,
    window_size: i32,
    omit_tail: bool,
    chrom_sizes_path: String,
    write_chrom_sizes: bool,
    verbose: bool,
) {

    let input = PathBuf::from(input);
    let output = PathBuf::from(output);
    let chrom_sizes_path = PathBuf::from(chrom_sizes_path);

    rust_gc_count::gc_count::write_gc_to_file(input, output, compression_level, window_size, omit_tail, chrom_sizes_path, write_chrom_sizes, verbose)
}
