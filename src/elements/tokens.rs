#[derive(Debug)]
pub enum Tokens {
    HeadingOne(&'static str),
    Newline(&'static str),
    Space(&'static str),
}
