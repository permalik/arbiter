use crate::elements::{literals, tokens::Tokens};

struct Token {
    kind: Tokens,
    value: String,
    position: usize,
}

pub fn tokenize(file_contents: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for (i, c) in file_contents.chars().enumerate() {
        match c {
            ' ' => {
                let space = literals::SPACE;
                if let Tokens::Space(ref space_literal) = space {
                    tokens.push({
                        Token {
                            kind: Tokens::Space(" "),
                            value: space_literal.to_string(),
                            position: i,
                        }
                    });
                    println!("space_token: {:?}", space_literal);
                }
            }
            '\n' => {
                let newline = literals::NEWLINE;
                if let Tokens::Newline(ref newline_literal) = newline {
                    tokens.push({
                        Token {
                            kind: Tokens::Newline("\n"),
                            value: newline_literal.to_string(),
                            position: i,
                        }
                    });
                    println!("newline_token: {:?}", newline_literal);
                }
            }
            _ => {}
        }
    }
    tokens
}
