use clap::Parser;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use seq_io::fasta::{Reader, Record};
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::io::BufWriter;

/// Parse a FASTA file and calculate GC based on the specified window into a wig file
#[derive(Parser)]
#[command(
    author, 
    name = "gccount", 
    version, 
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

    if args.verbose {
        eprintln!(
            "==> Processing FASTA file {:?} and writing to {:?}",
            args.input, args.output
        );
    }

    let read = if args.input.extension().unwrap() == "gz" {
        Box::new(MultiGzDecoder::new(File::open(args.input).unwrap())) as Box<dyn Read>
    } else {
        Box::new(File::open(args.input).unwrap()) as Box<dyn Read>
    };
    let mut reader = Reader::new(read);
    let write: Box<dyn Write> = if args.output.extension().unwrap() == "gz" {
        Box::new(GzEncoder::new(
            File::create(args.output).expect("creation failed"),
            Compression::new(args.compression_level),
        )) as Box<dyn Write>
    } else {
        Box::new(File::create(args.output).expect("creation failed")) as Box<dyn Write>
    };
    let mut writer = BufWriter::new(write);

    if args.verbose && args.write_chrom_sizes {
        eprintln!(
            "==> Will write chrom.sizes file to {:?}",
            args.chrom_sizes_path
        );
    }
    let mut chrom_sizes_writer = if args.write_chrom_sizes {
        let tmp_writer =
            BufWriter::new(File::create(args.chrom_sizes_path).expect("Creation failed"));
        Some(tmp_writer)
    } else {
        None
    };

    let window_size: i32 = args.window;

    writer
        .write("track type=wiggle_0\n".as_bytes())
        .expect("Write failed");
    let mut n = 0;
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let id = record.id().unwrap();
        if args.verbose {
            eprint!("==> Processing region {:?} ... ", id);
        }
        // variabletep is used versus fixedStep because there are differences in the
        // resulting BigWigs. Using variableStep keeps this inline with UCSC
        // gc BigWig files
        writer
            .write(format!("variableStep chrom={0} span={1}\n", id, window_size).as_bytes())
            .expect("Write failed");
        let mut length = 0;
        let mut gc_count = 0;
        let mut window_count = 0_i32;
        let mut iter_count = 0_i32;
        for s in record.seq_lines() {
            for c in s.iter() {
                if *c == b'C' || *c == b'G' {
                    gc_count += 1;
                }
                window_count += 1;
                if window_count == window_size {
                    iter_count += 1;
                    write_gc(gc_count, window_size, iter_count, &mut writer);
                    gc_count = 0;
                    window_count = 0;
                }
            }
            length += s.len();
        }
        if !args.omit_tail && window_count > 0 {
            write_gc(gc_count, window_count, iter_count, &mut writer);
        }

        if chrom_sizes_writer.is_some() {
            chrom_sizes_writer
                .as_mut()
                .unwrap()
                .write(format!("{0}\t{1}\n", record.id().unwrap(), length).as_bytes())
                .expect("Write failed");
        }
        n += 1;
        if args.verbose {
            eprintln!("done");
        }
    }
    writer.flush().expect("Could not close wig output file");
    if chrom_sizes_writer.is_some() {
        chrom_sizes_writer
            .as_mut()
            .unwrap()
            .flush()
            .expect("Could not close chrom sizes stream");
    }
    if args.verbose {
        eprintln!("==> Found and processed {} regions.", n);
    }
}

fn write_gc(gc_count: i32, window_size: i32, iter_count: i32, writer: &mut impl Write) {
    let gc = (gc_count as f32 / window_size as f32) * 100_f32;
    let start_location = ((iter_count - 1) * window_size) + 1;
    writer
        .write(format!("{}\t{}\n", start_location, gc as i32).as_bytes())
        .expect("Write failed");
}
