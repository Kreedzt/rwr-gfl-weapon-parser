use std::fs::File;
use std::io::{Bytes, Read};
use std::path::{Path, PathBuf};
use encoding_rs::*;

pub fn read_file_decode_to_utf8(path: &str) -> String {
    let mut file = File::open(path).expect("err");

    let mut buf_string = String::new();
    let bytes = file.read_to_string(&mut buf_string).unwrap();

    let (cow, encoding_used) = UTF_8.decode_with_bom_removal(buf_string.as_bytes());

    String::from(&cow[..])

    // let mut buffer_bytes = [0u8; 2048];
    // let mut buffer: &mut str = std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap();
    //
    // let mut bytes_in_buffer = 0usize;
    //
    // let mut output = String::new();
    //
    // let mut decoder = UTF_8.new_decoder_with_bom_removal();
    //
    //
}