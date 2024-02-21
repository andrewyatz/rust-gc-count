use clap::Parser;
use rust_gc_count::gc_count::write_gc_to_file;

/// Parse a FASTA file and calculate GC based on the specified window into a wig file
#[derive(Parser)]
#[command(
    author = "Andrew Yates",
    name = "gccount", 
    version = env!("CARGO_PKG_VERSION"), 
    about = "Calculate GC and write into a wiggle file", 
    long_about = None
)]
struct Cli {
    /// FASTA formatted file to calculate GC from. Reads gzipped FASTA if the filename ends with .gz (including bgzip files)
    #[arg(long, value_name = "INPUT")]
    input: std::path::PathBuf,
    /// Output wiggle file. One file will be produced. Will be gzipped on the fly if the supplied filename ends with .gz
    #[arg(long, value_name = "OUTPUT")]
    output: std::path::PathBuf,
    /// Gzip compression level to use for writing. Set between 0 (no compression) to 9 (max compression).
    #[arg(long, value_name = "LEVEL", default_value_t = 5)]
    compression_level: u32,
    /// Window size to calculate GC over
    #[arg(long, default_value_t = 5)]
    window: i32,
    /// Remove any trailing sequence and do not calcualte GC. Default behaviour is to retain the leftover sequence. GC is calculated over the remaining sequence length
    #[arg(long, default_value_t = false)]
    omit_tail: bool,
    /// Write a chrom.sizes file into the current directory. Use --chrom-sizes-path to configure location
    #[arg(long, default_value_t = false)]
    write_chrom_sizes: bool,
    /// Path of the chrom.sizes file. Defaults to chrom.sizes
    #[arg(long, value_name = "CHROM.SIZES", default_value = "chrom.sizes")]
    chrom_sizes_path: std::path::PathBuf,
    /// Be verbose
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    let input = args.input;
    let output = args.output;
    let compression_level = args.compression_level;
    let window_size = args.window;
    let omit_tail = args.omit_tail;
    let chrom_sizes_path = args.chrom_sizes_path;
    let write_chrom_sizes = args.write_chrom_sizes;
    let verbose = args.verbose;

    if args.verbose {
        eprintln!(
            "==> Processing FASTA file {:?} and writing to {:?}",
            input, output
        );
    }

    write_gc_to_file(
        input,
        output,
        compression_level,
        window_size,
        omit_tail,
        chrom_sizes_path,
        write_chrom_sizes,
        verbose,
    );
}
