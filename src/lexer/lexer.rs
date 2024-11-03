use crate::elements::{literals, tokens::Tokens};

pub struct Token {
    kind: Tokens,
    value: String,
    position: usize,
}

pub fn parse(file_contents: &str) {
    for token in tokenize(file_contents) {
        println!(
            "Token: kind: {:?}, value: {:?}, position: {:?}",
            token.kind, token.value, token.position
        );
    }
}

pub fn tokenize(file_contents: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for (i, c) in file_contents.chars().enumerate() {
        match c {
            ' ' => {
                let space = literals::SPACE;
                if let Tokens::Space(ref space_literal) = space {
                    assert_eq!(space_literal, &" ");
                    tokens.push({
                        Token {
                            kind: Tokens::Space(" "),
                            value: space_literal.to_string(),
                            position: i,
                        }
                    });
                }
            }
            '\n' => {
                let newline = literals::NEWLINE;
                if let Tokens::Newline(ref newline_literal) = newline {
                    assert_eq!(newline_literal, &"  ");
                    tokens.push({
                        Token {
                            kind: Tokens::Newline("  "),
                            value: newline_literal.to_string(),
                            position: i,
                        }
                    });
                }
            }
            _ => {}
        }
    }
    tokens
}
