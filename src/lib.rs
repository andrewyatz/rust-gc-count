use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use md5::{Digest, Md5};
use seq_io::fasta::{Reader, Record, RefRecord};
use sha2::Sha512;
use std::fs::File;
use std::io::prelude::{Read, Write};

use std::io::BufWriter;

pub mod checksum {

    use super::*;

    pub fn process_sequence(record: RefRecord, verbose: bool) -> ChecksumResult {
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

        ChecksumResult {
            id: id.to_string(),
            length,
            sha512,
            md5,
        }
    }

    pub struct ChecksumResult {
        pub id: String,
        pub length: usize,
        pub sha512: String,
        pub md5: String,
    }
}

pub mod gc_count {

    use std::path::PathBuf;

    use super::*;

    pub fn write_gc(gc_count: i32, window_size: i32, iter_count: i32, writer: &mut impl Write) {
        let gc = (gc_count as f32 / window_size as f32) * 100_f32;
        let start_location = ((iter_count - 1) * window_size) + 1;
        writer
            .write_all(format!("{}\t{}\n", start_location, gc as i32).as_bytes())
            .expect("Write failed");
    }

    pub fn write_gc_to_file(
        input: PathBuf,
        output: PathBuf,
        compression_level: u32,
        window_size: i32,
        omit_tail: bool,
        chrom_sizes_path: PathBuf,
        write_chrom_sizes: bool,
        verbose: bool,
    ) {
        let read = if input.extension().unwrap() == "gz" {
            Box::new(MultiGzDecoder::new(File::open(input).unwrap())) as Box<dyn Read>
        } else {
            Box::new(File::open(input).unwrap()) as Box<dyn Read>
        };
        let mut reader = Reader::new(read);
        let write: Box<dyn Write> = if output.extension().unwrap() == "gz" {
            Box::new(GzEncoder::new(
                File::create(output).expect("creation failed"),
                Compression::new(compression_level),
            )) as Box<dyn Write>
        } else {
            Box::new(File::create(output).expect("creation failed")) as Box<dyn Write>
        };
        let mut writer = BufWriter::new(write);

        if verbose && write_chrom_sizes {
            eprintln!("==> Will write chrom.sizes file to {:?}", chrom_sizes_path);
        }
        let mut chrom_sizes_writer = if write_chrom_sizes {
            let tmp_writer =
                BufWriter::new(File::create(chrom_sizes_path).expect("Creation failed"));
            Some(tmp_writer)
        } else {
            None
        };

        writer
            .write_all("track type=wiggle_0\n".as_bytes())
            .expect("Write failed");
        let mut n = 0;
        while let Some(record) = reader.next() {
            let record = record.expect("Error reading record");
            let id = record.id().unwrap();
            if verbose {
                eprint!("==> Processing region {:?} ... ", id);
            }
            // variabletep is used versus fixedStep because there are differences in the
            // resulting BigWigs. Using variableStep keeps this inline with UCSC
            // gc BigWig files
            writer
                .write_all(format!("variableStep chrom={0} span={1}\n", id, window_size).as_bytes())
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
            if !omit_tail && window_count > 0 {
                write_gc(gc_count, window_count, iter_count, &mut writer);
            }

            if chrom_sizes_writer.is_some() {
                chrom_sizes_writer
                    .as_mut()
                    .unwrap()
                    .write_all(format!("{0}\t{1}\n", record.id().unwrap(), length).as_bytes())
                    .expect("Write failed");
            }
            n += 1;
            if verbose {
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
        if verbose {
            eprintln!("==> Found and processed {} regions.", n);
        }
    }
}
