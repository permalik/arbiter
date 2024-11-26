use crate::elements::tokens::Tokens;

pub struct Token {
    pub line_number: usize,
    pub name: String,
    pub kind: Tokens,
    pub value: String,
}
