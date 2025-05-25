use crate::{objects::Blob, traits::ObjectDeserialize};

pub fn cat_file(sha1: &str, flag: &str) -> String {
    let data = Blob::deserialize(sha1);
    match flag {
        "-t" => format!("{}", data[0]),
        "-s" => format!("{}", data[1]),
        "-p" => format!("{}", data[2]),
        _ => panic!("[ERROR] Invalid flag provided for cat-file argument."),
    }
}
