use crate::utils::generate_sha1;
use crate::objects::Blob;
use crate::traits::ObjectSerialize;

pub fn hash_object(from_stdin: bool, input: &str) -> String {
    match from_stdin {
        // If true, the input is a string.
        true => generate_sha1(input),
        // If false, the input is a file path that needs to be encrypted.
        false => {
            let blob = Blob::new();
            blob.serialize(input)
        }
    }
}
