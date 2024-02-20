use sha2::Sha512;
use md5::{Digest, Md5};
use seq_io::fasta::{Record, RefRecord};

pub mod checksum {

    use super::*;

    pub fn process_sequence(record: RefRecord, verbose: bool) -> (String, usize, String, String) {
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
        
        (id.to_string(), length, sha512, md5)
    }
}
