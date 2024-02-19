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
Iterates through a FASTA file calclating checksums and sequence length

Usage: checksumseq [OPTIONS]

Options:
      --input <INPUT>    FASTA formatted file to calculate checksums from (- mean STDIN). Reads gzipped FASTA if the filename ends with .gz (including bgzip files) [default: -]
      --output <OUTPUT>  Output file (- means STDOUT). Each line is tab separated reporting "ID Length sha512t24u md5" [default: -]
      --verbose          Be verbose
  -h, --help             Print help
  -V, --version          Print version
```

### From within Python
Python bindings are available for the checksumseq calculation. The following code demonstrates how to use the bindings.

#### Install the bindings
[`maturin`](https://github.com/PyO3/maturin) is used to build the bindings and install them into the current environment. Ensure you are using the Python environment you want to install the bindings into.
```bash
pip install maturin
```
Then navigate to the `rust-gc-count/bindings` directory and run the following command to install the bindings.
```bash
maturin build --release
```

#### Use the bindings
To use the bindings in Python, the following code demonstrates how to use the bindings.
```python
from gc_count import checksum

results = checksum("path/to/seq/fasta")
for result in results:
    print(result.sha512)
```

## Level of code quality

The code developed here has not been extensively tested but has been verified as producing correct and expected output.
