use std::{
    collections::HashMap,
    fs,
    io::{BufReader, Read},
};

use flate2::read::ZlibDecoder;
use hex::encode;
use regex::Regex;
use sha1::{Digest, Sha1};

use crate::constants::COMMIT_MESSAGE;

pub fn generate_sha1(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let mut res: [u8; 20] = [0; 20];
    hasher.finalize_into((&mut res).into());
    format!("{}", encode(res))
}

pub fn zlib_decompress(sha1: &str, buffer: &mut Vec<u8>) {
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

pub fn kvlm_parse(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    // The regex to match the commit message key-value format.
    let re = Regex::new(r"(?m)^(\S+)\s(.*(?:\n(?:\s+.*))*)").unwrap();

    for capture in re.captures_iter(content) {
        let key = capture[1].to_string();
        let value = capture[2].replace("\n ", "\n");

        map.entry(key)
            .and_modify(|v| *v = format!("{v}\n{value}"))
            .or_insert(value);
    }

    // Commit message.
    if let Some(pos) = content.find("\n\n") {
        let commit_message = content[pos + 2..].to_string();
        map.insert(COMMIT_MESSAGE.to_string(), commit_message);
    }

    map
}
