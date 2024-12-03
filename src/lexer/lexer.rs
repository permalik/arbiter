use crate::elements::{literals, structs::Token, tokens::Tokens};

pub fn lex(line_number: usize, line: &str, byte_offset: &mut usize, tokens: &mut Vec<Token>) {
    if line.is_empty() {
        let empty_line_literal = literals::EMPTY_LINE;
        if let Tokens::EmptyLine(empty_line) = empty_line_literal {
            if line == empty_line {
                tokens.push(Token {
                    line_number,
                    name: "empty_line".to_string(),
                    kind: Tokens::EmptyLine(""),
                    value: "".to_string(),
                    line_byte_offset: *byte_offset,
                });
            }
        }
    }
    for (_, c) in line.chars().enumerate() {
        *byte_offset += c.len_utf8();
        match c {
            '#' => {
                fn verify_heading(level: usize, literal: Tokens) {
                    let expected = format!("{} ", &"#".repeat(level));
                    match literal {
                        Tokens::HeadingOne(h) => assert_eq!(h, expected),
                        Tokens::HeadingTwo(h) => assert_eq!(h, expected),
                        Tokens::HeadingThree(h) => assert_eq!(h, expected),
                        Tokens::HeadingFour(h) => assert_eq!(h, expected),
                        Tokens::HeadingFive(h) => assert_eq!(h, expected),
                        Tokens::HeadingSix(h) => assert_eq!(h, expected),
                        _ => unreachable!(),
                    }
                }

                let mut heading_level = 1;
                while heading_level < line.len() && line.chars().nth(heading_level) == Some('#') {
                    heading_level += 1;
                }

                if heading_level <= 6 {
                    let (token_kind, literal) = match heading_level {
                        1 => (Tokens::HeadingOne("# "), literals::HEADING_ONE),
                        2 => (Tokens::HeadingTwo("## "), literals::HEADING_TWO),
                        3 => (Tokens::HeadingThree("### "), literals::HEADING_THREE),
                        4 => (Tokens::HeadingFour("#### "), literals::HEADING_FOUR),
                        5 => (Tokens::HeadingFive("##### "), literals::HEADING_FIVE),
                        6 => (Tokens::HeadingSix("######"), literals::HEADING_SIX),
                        _ => unreachable!(),
                    };

                    verify_heading(heading_level, literal);

                    let heading_text = lex_element_text(line, heading_level);

                    tokens.push(Token {
                        line_number,
                        name: format!("h{}", heading_level),
                        kind: token_kind,
                        value: format!("{} {}", "#".repeat(heading_level), heading_text),
                        line_byte_offset: *byte_offset,
                    });
                    return;
                }
            }
            _ => {
                lex_text(line_number, c, byte_offset, tokens);
                return;
            }
        }
    }
}

fn lex_element_text(line: &str, heading_level: usize) -> &str {
    &line[heading_level..]
}

fn lex_text(line_number: usize, c: char, offset: &mut usize, tokens: &mut Vec<Token>) {
    tokens.push(Token {
        line_number,
        name: "text".to_string(),
        kind: Tokens::Text(String::from(c)),
        value: String::from(c),
        line_byte_offset: *offset,
    });
}
