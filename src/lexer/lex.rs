use crate::elements::{literals, tokens::Tokens};

pub fn tokenize(file_contents: String) {
    for c in file_contents.chars() {
        match c {
            ' ' => {
                let space = literals::SPACE;
                if let Tokens::Space(ref space_literal) = space {
                    println!("space_token: {:?}", space_literal);
                }
            }
            '\n' => {
                let newline = literals::NEWLINE;
                if let Tokens::Newline(ref newline_literal) = newline {
                    println!("newline_token: {:?}", newline_literal);
                }
            }
            _ => {}
        }
    }
}
