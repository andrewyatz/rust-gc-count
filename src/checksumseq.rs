use base64_url;
use clap::Parser;
use flate2::read::MultiGzDecoder;
use md5::{Digest, Md5};
use seq_io::fasta::{Reader, Record, RefRecord};
use sha2::Sha512;
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::io::BufWriter;
use std::io::{stdin, stdout};

/// Parse a FASTA file and calculate checksums for each record
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Iterates through a FASTA file calclating checksums and sequence length"
)]
struct Cli {
    /// FASTA formatted file to calculate checksums from (- mean STDIN). Reads gzipped FASTA if the filename ends with .gz (including bgzip files)
    #[arg(long, value_name = "INPUT", default_value = "-")]
    input: std::path::PathBuf,
    /// Output file (- means STDOUT). Each line is tab separated reporting "ID Length sha512t24u md5"
    #[arg(long, value_name = "OUTPUT", default_value = "-")]
    output: std::path::PathBuf,
    /// Be verbose
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    if args.verbose {
        println!(
            "==> Processing FASTA file {:?} and writing to {:?}",
            args.input, args.output
        );
    }

    let read = if args.input.to_str().unwrap() == "-" {
        Box::new(stdin()) as Box<dyn Read>
    } else if args.input.extension().unwrap() == "gz" {
        Box::new(MultiGzDecoder::new(File::open(args.input).unwrap())) as Box<dyn Read>
    } else {
        Box::new(File::open(args.input).unwrap()) as Box<dyn Read>
    };
    let mut reader = Reader::new(read);

    let writer = if args.output.to_str().unwrap() == "-" {
        Box::new(stdout()) as Box<dyn Write>
    } else {
        Box::new(File::create(args.output).expect("Cannot open file for writing")) as Box<dyn Write>
    };
    let mut writer = BufWriter::new(writer);

    let mut n = 0;

    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let result = process_sequence(record, args.verbose);
        let line = format!("{0:#}\t{1:#}\tSQ.{2:#}\t{3:#}\n", result.0, result.1, result.2, result.3);
        writer
            .write(line.as_bytes())
            .expect("Could not write to file");
        if args.verbose {
            eprintln!("done");
        }
        n += 1;
    }
    if args.verbose {
        eprintln!("==> Found and processed {} regions.", n);
    }
    writer.flush().expect("Could not flush writer");
}

fn process_sequence(record: RefRecord, verbose: bool) -> (String, usize, String, String) {
    let mut md5_hasher_box = Box::new(Md5::new());
    let mut sha512_hasher_box = Box::new(Sha512::new());
    let id = record.id().unwrap();
    let mut length = 0;
    if verbose {
        eprint!("==> Processing region {:?} ... ", id);
    }
    for s in record.seq_lines() {
        sha512_hasher_box.as_mut().update(s.to_ascii_uppercase());
        md5_hasher_box.as_mut().update(s.to_ascii_uppercase());
        length += s.len();
    }
    let sha512 = base64_url::encode(&sha512_hasher_box.as_mut().finalize_reset()[0..24]);
    let md5 = format!("{:x}", md5_hasher_box.as_mut().finalize_reset());
    return (id.to_string(), length, sha512, md5);
}

#[test]
fn it_works() {
    let fasta: &[u8] = b"
>id basic\n
ACGT\n
>id second\n
acgT\n
";
    let mut reader = Reader::new(fasta);
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let result = process_sequence(record, false);
        assert_eq!(result.0, "id");
        assert_eq!(result.1, 4);
        assert_eq!(result.2, "aKF498dAxcJAqme6QYQ7EZ07-fiw8Kw2");
        assert_eq!(result.3, "f1f8f4bf413b16ad135722aa4591043e");
    }
}
