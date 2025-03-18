use std::fs;

pub fn init() {
    fs::create_dir(".gitrs").unwrap();
    fs::create_dir(".gitrs/objects").unwrap();
    fs::create_dir(".gitrs/refs").unwrap();
    fs::write(".gitrs/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized gitrs directory")
}