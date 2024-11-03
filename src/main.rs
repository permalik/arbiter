mod elements;
mod lexer;

use lexer::lex;
use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);

    match read_file(file_path) {
        Ok(file_contents) => lex::tokenize(file_contents),
        Err(e) => eprintln!("error: {e}"),
    }
}

fn parse_args(args: &[String]) -> Option<&str> {
    if args.len() > 1 {
        Some(&args[1])
    } else {
        None
    }
}

fn read_file(file_path: Option<&str>) -> Result<String, Box<dyn Error>> {
    match file_path {
        Some(path) => fs::read_to_string(path).map_err(|e| e.into()),
        None => Err("no file path provided".into()),
    }
}
