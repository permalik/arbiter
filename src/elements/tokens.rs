#[derive(Debug)]
pub enum Tokens {
    HeadingOne(&'static str),
    HeadingTwo(&'static str),
    HeadingThree(&'static str),
    HeadingFour(&'static str),
    HeadingFive(&'static str),
    HeadingSix(&'static str),
    Newline(&'static str),
    Space(&'static str),
}
