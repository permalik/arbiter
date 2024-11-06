//mod elements;
//mod lexer;

//use lexer::lexer::parse;
//use std::error::Error;
//use std::fs::File;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use std::{env, io};
//use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let file_path = parse_args(&args);

    if let Ok(lines) = read_lines("./test.md") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }

    //match read_file(file_path) {
    //    Ok(file_contents) => {
    //        let file_contents_str: &str = &file_contents;
    //        parse(file_contents_str);
    //    }
    //    Err(e) => eprintln!("error: {e}"),
    //}
}

//fn parse_args(args: &[String]) -> Option<&str> {
//    if args.len() > 1 {
//        Some(&args[1])
//    } else {
//        None
//    }
//}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

//fn read_file(file_path: Option<&str>) -> Result<String, Box<dyn Error>> {
//    match file_path {
//        Some(path) => fs::read_to_string(path).map_err(|e| e.into()),
//        None => Err("no file path provided".into()),
//    }
//}
