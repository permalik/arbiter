use crate::elements::{structs::Token, tokens::Tokens};

pub fn parse(tokens: Vec<Token>) {
    for token in tokens {
        match &token.kind {
            Tokens::Text(content) => {
                let _ = content.len();
            }
            _ => {}
        }
        println!(
            "Token:: line_number: {:?}\nname: {:?}\nkind: {:?}\nvalue: {:?}\n",
            token.line_number, token.name, token.kind, token.value,
        );
    }
}
