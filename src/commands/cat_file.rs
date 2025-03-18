use std::{
    fs,
    io::{BufReader, Read},
};

use flate2::read::ZlibDecoder;
use regex::Regex;

pub fn cat_file(sha1: &str, flag: &str) -> String {
    let mut encrypted_content: Vec<u8> = Vec::new();
    zlib_decompress(sha1, &mut encrypted_content);
    return parse_file(encrypted_content, flag);
}

fn zlib_decompress(sha1: &str, buffer: &mut Vec<u8>) {
    let folder = &sha1[..2];
    let file = &sha1[2..];
    let raw_content = fs::File::open(format!(".gitrs/objects/{folder}/{file}"))
        .expect("[ERROR] Couldn't read blob file.");
    let decoder = ZlibDecoder::new(raw_content);
    let mut decoder = BufReader::new(decoder);
    decoder
        .read_to_end(buffer)
        .expect("[ERROR] Couldn't read to string.");
}

fn parse_file(encrypted_content: Vec<u8>, flag: &str) -> String {
    let decrypted_content = encrypted_content
        .iter()
        .map(|&c| c as char)
        .collect::<String>();

    let decrypted_content = decrypted_content.replace("\r\n", "");

    let type_regex = Regex::new(r"^([a-zA-Z]+) ").expect("[ERROR] Type regex creation error");
    let size_regex = Regex::new(r" ([0-9]+)\x00").expect("[ERROR] Size regex creation error");
    let content_regex = Regex::new(r"\x00(.*)$").expect("[ERROR] Content regex creation error");

    let obj_type = type_regex
        .captures(&decrypted_content)
        .expect("[ERROR] Couldn't find type");
    let size = size_regex
        .captures(&decrypted_content)
        .expect("[ERROR] Couldn't find size");
    let content = content_regex
        .captures(&decrypted_content)
        .expect("[ERROR] Couldn't find content");

    match flag {
        "-t" => format!("{}", &obj_type[1]),
        "-s" => format!("{}", &size[1]),
        "-p" => format!("{}", &content[1]),
        _ => panic!("[ERROR] Invalid flag provided for cat-file argument."),
    }
}
