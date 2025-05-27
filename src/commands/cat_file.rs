use crate::{objects::Blob, objects::GitObject};

pub fn cat_file(sha1: &str, flag: &str) -> String {
    let mut blob = Blob::new(format!(".gitrs/objects/{}/{}", &sha1[..2], &sha1[2..]));
    blob.deserialize(());
    match flag {
        "-t" => format!("{}", blob.to_string()),
        "-s" => format!("{}", blob.size),
        "-p" => format!("{}", blob.content),
        _ => panic!("[ERROR] Invalid flag provided for cat-file argument."),
    }
}
