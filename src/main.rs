use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path_ref = parse_args(&args);
    let file_path: Option<String> = file_path_ref.map(str::to_string);
    // let file_contents = fs::read_to_string(file_path.unwrap()).expect("could not read the file");

    match file_path {
        Some(path) => match fs::read_to_string(&path) {
            Ok(file_contents) => println!("{file_contents}"),
            Err(e) => eprintln!("failed to read file: {e}"),
        },
        None => eprintln!("usage: cargo run -- file_path.md"),
    }
}

fn parse_args(args: &[String]) -> Option<&str> {
    if args.len() > 1 {
        Some(&args[1])
    } else {
        None
    }
}
