use std::fs::{create_dir, write, File};
use std::io::{BufReader, Read, Write};

use flate2::{write::ZlibEncoder, Compression};
use hex::encode;
use sha1::{Digest, Sha1};

pub fn hash_object(from_stdin: bool, input: &str) -> String {
    match from_stdin {
        // If true, the input is a string.
        true => generate_sha1(input),
        // If false, the input is a file path that needs to be encrypted.
        false => {
            // Opening the file containing all the content that needs to be versioned.
            // !PLEASE FOR THE LOVE OF GOD MAKE SURE THAT THE TEXT FILE IS UTF-8 ENCODED AND NOT ANYTHING ELSE LIKE UTF8 WITH BOM OR UTF16 LE OR SOME SHIT.
            let file = File::open(input)
                .expect("[ERROR] Couldn't read the file that needs to be compressed.");
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
}

fn generate_sha1(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let mut res: [u8; 20] = [0; 20];
    hasher.finalize_into((&mut res).into());
    format!("{}", encode(res))
}
