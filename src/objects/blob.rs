use flate2::{write::ZlibEncoder, Compression};
use regex::Regex;
use std::{
    fs,
    io::{BufReader, Read, Write},
};

use crate::{
    objects::GitObject,
    utils::{generate_sha1, zlib_decompress},
};

#[derive(Default)]
pub struct Blob {
    pub path: String,
    pub size: usize,
    pub content: String,
}

impl Blob {
    pub fn new(path: String) -> Self {
        Self { path: path, size: Default::default(), content: Default::default() }
    }
}

impl ToString for Blob {
    fn to_string(&self) -> String {
        String::from("blob")
    }
}

impl GitObject for Blob {
    type SerializerArg<'a> = &'a str;
    type DeserializerArg<'b> = ();

    fn serialize(&self, path: &str) -> String {
        // Opening the file containing all the content that needs to be versioned.
        // !PLEASE FOR THE LOVE OF GOD MAKE SURE THAT THE TEXT FILE IS UTF-8 ENCODED AND NOT ANYTHING ELSE LIKE UTF8 WITH BOM OR UTF16 LE OR SOME SHIT.
        let file =
            fs::File::open(path).expect("[ERROR] Couldn't read the file that needs to be compressed.");
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
        fs::create_dir(format!(".gitrs/objects/{}", &sha1[..2])).unwrap();
        fs::write(
            format!(".gitrs/objects/{}/{}", &sha1[..2], &sha1[2..]),
            buffer,
        )
        .unwrap();

        sha1
    }

    fn deserialize(&mut self, _: ()) {
        let mut encrypted_content: Vec<u8> = Vec::new();
        zlib_decompress(&self.path, &mut encrypted_content);

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

        // Blob {
        //     path: self.path.clone(),
        //     size: size[1].parse::<usize>().expect("[Error] Could not parse size"),
        //     content: String::from(&content[1]),
        // }

        self.size = size[1].parse::<usize>().expect("[Error] Could not parse size");
        self.content = String::from(&content[1]);
    }
}
