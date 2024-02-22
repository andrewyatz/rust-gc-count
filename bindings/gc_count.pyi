from typing import List

class ChecksumResult:
    def __init__(self, id: str, length: int, sha512: str, md5: str):
        self.id = id
        self.length = length
        self.sha512 = sha512
        self.md5 = md5

    def __repr__(self):
        return f"ChecksumResult(id={self.id}, length={self.length}, sha512={self.sha512}, md5={self.md5})"

    def __str__(self):
        return f"ChecksumResult(id={self.id}, length={self.length}, sha512={self.sha512}, md5={self.md5})"

def checksum(file: str, verbose: bool) -> List[ChecksumResult]:
    """
    Calculate the sequence lengths and checksums from a fasta file. It will
    produce a list of ChecksumResult objects, each containing the following
    
    Sequence ID as it appears in the FASTA file
    Sequence length
    Refget ga4gh identifier (SQ.sha512t24u)
    MD5 checksum hex encoded

    :param file: The file to checksum
    :param verbose: Whether to print out the progress
    """

def write_gc_count_to_file(
    input: str,
    output: str,
    compression_level: int,
    window_size: int,
    omit_tail: bool,
    chrom_sizes_path: str,
    write_chrom_sizes: bool,
    verbose: bool,
) -> None:
    """
    Calculate the GC content and write it to a file. The file will be a
    tab-separated file with the following columns:
    
    Chromosome name
    Start position
    End position
    GC content

    :param input: The input file to calculate the GC content from
    :param output: The output file to write the GC content to
    :param compression_level: The compression level to use for the output file
    :param window_size: The window size to use for calculating the GC content
    :param omit_tail: Whether to omit the tail of the sequence
    :param chrom_sizes_path: The path to the chromosome sizes file
    :param write_chrom_sizes: Whether to write the chromosome sizes to the output file
    :param verbose: Whether to print out the progress
    """       