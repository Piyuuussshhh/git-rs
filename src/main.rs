use std::env;

mod commands;
use commands::{cat_file, hash_object, init, Commands};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {args:?}");
    match Commands::from(&args[1]) {
        Commands::Init => init(),
        Commands::HashObject => print!(
            "{}",
            hash_object(
                if args.contains(&"--stdin".to_string()) {
                    true
                } else {
                    false
                },
                if args.len() > 3 {
                    &args[3]
                } else {
                    panic!("[ERROR] Wtf gimme a string?")
                },
            )
        ),
        Commands::CatFile => print!(
            "{}",
            cat_file(
                if args.len() > 3 {
                    &args[3]
                } else {
                    panic!("[ERROR] Wtf gimme a sha1 string?")
                },
                if args.contains(&"-p".to_string()) {
                    "-p"
                } else if args.contains(&"-s".to_string()) {
                    "-s"
                } else if args.contains(&"-t".to_string()) {
                    "-t"
                } else {
                    "error"
                }
            )
        ),
        Commands::Err => panic!("[ERROR] Please provide a valid command."),
    }
}
