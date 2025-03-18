pub mod init;
pub use init::init;

pub mod hash_object;
pub use hash_object::hash_object;

pub mod cat_file;
pub use cat_file::cat_file;

pub enum Commands {
    Init,
    HashObject,
    CatFile,
    Err,
}

impl From<&String> for Commands {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "init" => Commands::Init,
            "hash-object" => Commands::HashObject,
            "cat-file" => Commands::CatFile,
            _ => Commands::Err,
        }
    }
}

impl Into<String> for Commands {
    fn into(self) -> String {
        match self {
            Commands::Init => String::from("init"),
            Commands::HashObject => String::from("hash-object"),
            Commands::CatFile => String::from("cat-file"),
            Commands::Err => String::from("[ERROR]"),
        }
    }
}
