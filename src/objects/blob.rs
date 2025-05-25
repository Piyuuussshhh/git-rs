use flate2::{write::ZlibEncoder, Compression};
use regex::Regex;
use std::{
    fs::{create_dir, write, File},
    io::{BufReader, Read, Write},
};

use crate::{
    traits::{ObjectDeserialize, ObjectSerialize},
    utils::{generate_sha1, zlib_decompress},
};

pub struct Blob;

impl Blob {
    pub fn new() -> Self {
        Blob {}
    }
}

impl ObjectSerialize for Blob {
    fn serialize(&self, path: &str) -> String {
        // Opening the file containing all the content that needs to be versioned.
        // !PLEASE FOR THE LOVE OF GOD MAKE SURE THAT THE TEXT FILE IS UTF-8 ENCODED AND NOT ANYTHING ELSE LIKE UTF8 WITH BOM OR UTF16 LE OR SOME SHIT.
        let file =
            File::open(path).expect("[ERROR] Couldn't read the file that needs to be compressed.");
        let mut buf = BufReader::new(file);
        let mut content = String::new();
        let size = buf
            .read_to_string(&mut content)
            .expect("[ERROR] Couldn't read the file");

        // Generating sha1 of the file's content PLUS the header. (sha1_content = file's content PLUS the header)
        let mut sha1_content = String::from("blob ") + size.to_string().as_str() + "\0";
        sha1_content.push_str(&content);
        let sha1 = generate_sha1(&sha1_content);

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        match encoder.write_all(sha1_content.as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }
        let buffer = encoder
            .finish()
            .expect("[ERROR] Couldn't compress header+content");

        // Creating the git object file.
        create_dir(format!(".gitrs/objects/{}", &sha1[..2])).unwrap();
        write(
            format!(".gitrs/objects/{}/{}", &sha1[..2], &sha1[2..]),
            buffer,
        )
        .unwrap();

        sha1
    }
}

impl ObjectDeserialize<[String; 3]> for Blob {
    fn deserialize(sha1: &str) -> [String; 3] {
        let mut encrypted_content: Vec<u8> = Vec::new();
        zlib_decompress(sha1, &mut encrypted_content);

        let decrypted_content = encrypted_content
            .iter()
            .map(|&c| c as char)
            .collect::<String>();

        let decrypted_content = decrypted_content.replace("\r\n", "");

        // let type_regex = Regex::new(r"^([a-zA-Z]+) ").expect("[ERROR] Type regex creation error");
        let size_regex = Regex::new(r" ([0-9]+)\x00").expect("[ERROR] Size regex creation error");
        let content_regex = Regex::new(r"\x00(.*)$").expect("[ERROR] Content regex creation error");

        // let obj_type = type_regex
        //     .captures(&decrypted_content)
        //     .expect("[ERROR] Couldn't find type");
        let size = size_regex
            .captures(&decrypted_content)
            .expect("[ERROR] Couldn't find size");
        let content = content_regex
            .captures(&decrypted_content)
            .expect("[ERROR] Couldn't find content");

        [
            String::from("blob"),
            String::from(&size[1]),
            String::from(&content[1]),
        ]
    }
}
