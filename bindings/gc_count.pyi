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