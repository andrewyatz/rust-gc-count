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

## Checksum calculator

```bash
target/release/checksumseq --input in.fa --output chrom.file
```

Another binary for calculating sequence lengths and checksums from a file. The resulting file is formted as tab separated with the following columns:

1. Sequence ID as it appears in the FASTA file
2. Sequence length
3. Refget ga4gh identifier (SQ.sha512t24u)
4. MD5 checksum hex encoded

The resulting file can be used as a `chrom.sizes` file too.

### Command line

```
Calculate GC and write into a wiggle file

Usage: checksumseq [OPTIONS] --input <INPUT>

Options:
      --input <INPUT>    FASTA formatted file to calculate checksums from. Reads gzipped FASTA if the filename ends with .gz (including bgzip files)
      --output <OUTPUT>  Output file (- means STDOUT). Each line is tab separated reporting "ID Length sha512t24u md5" [default: -]
      --verbose          Be verbose
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Level of code quality

The code developed here has not been extensively tested but has been verified as producing correct and expected output.
