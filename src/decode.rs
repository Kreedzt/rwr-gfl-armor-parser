use std::fs::File;
use std::io::Read;
use encoding_rs::*;
use anyhow::{Result, anyhow};

pub fn read_file_decode_to_utf8(path: &str) -> Result<String> {
    let file = File::open(path);

    match file {
        Ok(mut result) => {
            let mut buf_string = String::new();
            result.read_to_string(&mut buf_string).unwrap();

            let (cow, _) = UTF_8.decode_with_bom_removal(buf_string.as_bytes());

            Ok(String::from(&cow[..]))
        }
        Err(_) => Err(anyhow!("Can't open file"))
    }
}
