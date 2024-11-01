use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String;
    if args.len() > 1 {
        file_path = &args[1];

        println!("{}", file_path);
    } else {
        println!("usage: cargo run -- file_path.md");
    }
}
