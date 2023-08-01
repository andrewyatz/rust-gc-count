# rust-gc-count

```bash
cargo build --release
target/release/gccount --input in.fa --output out.wig
```

## Description

A tool for generating wiggle files of GC from DNA written in Rust.

## Help

```
Calculate GC and write into a wiggle file

Usage: gccount [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
      --input <INPUT>                   FASTA formatted file (can be gziped) to calculate GC from
      --output <OUTPUT>                 Output wiggle file. One file will be produced
      --window <WINDOW>                 Window size to calculate GC over [default: 5]
      --omit-tail                       Remove any trailing sequence and do not calcualte GC. Default behaviour is to retain the leftover sequence. GC is calculated over the remaining sequence length
      --write-chrom-sizes               Write a chrom.sizes file into the current directory. Use --chrom-sizes-path to configure location
      --chrom-sizes-path <CHROM.SIZES>  Path of the chrom.sizes file. Defaults to chrom.sizes [default: chrom.sizes]
      --verbose                         Be verbose
  -h, --help                            Print help
  -V, --version                         Print version
```
