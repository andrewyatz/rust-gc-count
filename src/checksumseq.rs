use clap::Parser;
use seq_io::fasta::{Reader,Record};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::io::BufWriter;
use sha2::Sha512;
use md5::{Md5, Digest};
use base64_url;
use std::io::{stdout,stdin};

/// Parse a FASTA file and calculate checksums for each record
#[derive(Parser)]
#[command(author, version, about = "Iterates through a FASTA file calclating checksums and sequence length")]
struct Cli {
    /// FASTA formatted file to calculate checksums from (- mean STDIN). Reads gzipped FASTA if the filename ends with .gz (including bgzip files)
    #[arg(long, value_name = "INPUT", default_value="-")]
    input: std::path::PathBuf,
    /// Output file (- means STDOUT). Each line is tab separated reporting "ID Length sha512t24u md5"
    #[arg(long, value_name = "OUTPUT", default_value="-")]
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
    }
    else {
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
      let mut md5_hasher_box = Box::new(Md5::new());
      let mut sha512_hasher_box = Box::new(Sha512::new());
      let record = record.expect("Error reading record");
      let id = record.id().unwrap();
      let mut length = 0;
      if args.verbose {
          eprint!("==> Processing region {:?} ... ", id);
      }
      for s in record.seq_lines() {
        sha512_hasher_box.as_mut().update(s);
        md5_hasher_box.as_mut().update(s);
        length += s.len();
      }
      n += 1;
      let sha512 = base64_url::encode(&sha512_hasher_box.as_mut().finalize_reset()[0..24]);
      let md5 = format!("{:x}", md5_hasher_box.as_mut().finalize_reset());
      let line = format!("{0}\t{1}\tSQ.{2}\t{3}\n", id, length, sha512, md5);
      writer.write(line.as_bytes()).expect("Could not write to file");
      if args.verbose {
        eprintln!("done");
      }
    }
    if args.verbose {
      eprintln!("==> Found and processed {} regions.", n);
    }
    writer.flush().expect("Could not flush writer");
}
