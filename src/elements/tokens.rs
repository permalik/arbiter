#[derive(Debug)]
pub enum Tokens {
    HeadingOne(&'static str),
    HeadingTwo(&'static str),
    HeadingThree(&'static str),
    HeadingFour(&'static str),
    HeadingFive(&'static str),
    HeadingSix(&'static str),
    UnorderedListHyphen(&'static str),
    Text(String),
    EmptyLine(&'static str),
    //LineBreak(&'static str),
}
