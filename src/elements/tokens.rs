#[derive(Debug)]
pub enum Tokens {
    HeadingOne(&'static str),
    HeadingTwo(&'static str),
    HeadingThree(&'static str),
    HeadingFour(&'static str),
    HeadingFive(&'static str),
    HeadingSix(&'static str),
    UnorderedListHyphen(&'static str),
    HorizontalRuleHyphen(&'static str),
    EmptyLine(&'static str),
    //LineBreak(&'static str),
    Text(String),
}
