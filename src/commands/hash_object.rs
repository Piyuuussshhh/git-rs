use crate::{
    objects::{Blob, GitObject},
    utils::generate_sha1,
};

pub fn hash_object(from_stdin: bool, input: &str) -> String {
    match from_stdin {
        // If true, the input is a string.
        true => generate_sha1(input),
        // If false, the input is a file path that needs to be encrypted.
        false => {
            /*
                The sole purpose of Blob::default() is because I need &self for Commit, but not here.
                Blob::serialize does not use the Blob object at all, therefore there
                shouldn't be any issue.
            */
            Blob::serialize(&Blob::default(), input)
        }
    }
}
