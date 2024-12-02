mod elements;
mod lexer;
mod parser;
mod utils;

use lexer::lexer::lex;
use parser::parser::parse;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);
    let mut tokens = Vec::new();

    match file_path {
        Some(path) => {
            if let Ok(lines) = read_lines(path.to_string()) {
                let mut byte_offset = 0;
                for (line_number, line) in lines.flatten().enumerate() {
                    lex(line_number + 1, &line, &mut byte_offset, &mut tokens);
                }
            }
        }
        None => panic!("no file path provided"),
    }

    parse(tokens);
}

fn parse_args(args: &[String]) -> Option<&str> {
    if args.len() > 1 {
        Some(&args[1])
    } else {
        None
    }
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}
