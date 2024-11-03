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
    let mut i = 0;

    while i < file_contents.len() {
        assert!(i < file_contents.len(), "assert: index oob");
        let c = file_contents.chars().nth(i).unwrap();
        match c {
            '#' => {
                let mut heading_level = 1;
                assert!(heading_level <= 6, "assert: heading_level exceeds 6");

                while i + heading_level < file_contents.len()
                    && file_contents.chars().nth(i + heading_level) == Some('#')
                {
                    heading_level += 1;
                    assert!(heading_level <= 6, "assert: heading_level exceeds 6");
                }

                if heading_level <= 6 {
                    let (_heading_literal, token_kind) = match heading_level {
                        1 => {
                            let heading_one = literals::HEADING_ONE;
                            if let Tokens::HeadingOne(ref heading_one_literal) = heading_one {
                                assert_eq!(heading_one_literal, &"#".repeat(heading_level));
                            }
                            (heading_one, Tokens::HeadingOne("#"))
                        }
                        2 => {
                            let heading_two = literals::HEADING_TWO;
                            if let Tokens::HeadingTwo(ref heading_two_literal) = heading_two {
                                assert_eq!(heading_two_literal, &"#".repeat(heading_level));
                            }
                            (heading_two, Tokens::HeadingTwo("##"))
                        }
                        3 => {
                            let heading_three = literals::HEADING_THREE;
                            if let Tokens::HeadingThree(ref heading_three_literal) = heading_three {
                                assert_eq!(heading_three_literal, &"#".repeat(heading_level));
                            }
                            (heading_three, Tokens::HeadingThree("###"))
                        }
                        4 => {
                            let heading_four = literals::HEADING_FOUR;
                            if let Tokens::HeadingFour(ref heading_four_literal) = heading_four {
                                assert_eq!(heading_four_literal, &"#".repeat(heading_level));
                            }
                            (heading_four, Tokens::HeadingFour("####"))
                        }
                        5 => {
                            let heading_five = literals::HEADING_FIVE;
                            if let Tokens::HeadingFive(ref heading_five_literal) = heading_five {
                                assert_eq!(heading_five_literal, &"#".repeat(heading_level));
                            }
                            (heading_five, Tokens::HeadingFive("#####"))
                        }
                        6 => {
                            let heading_six = literals::HEADING_SIX;
                            if let Tokens::HeadingSix(ref heading_six_literal) = heading_six {
                                assert_eq!(heading_six_literal, &"#".repeat(heading_level));
                            }
                            (heading_six, Tokens::HeadingSix("######"))
                        }
                        _ => unreachable!(),
                    };

                    tokens.push(Token {
                        kind: token_kind,
                        value: format!("{}", "#".repeat(heading_level)),
                        position: i,
                    });
                }
                i += heading_level;
                continue;
            }
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
                i += 1;
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
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    assert!(
        i > 0,
        "assert: index i did not advance, potential infinite loop"
    );
    tokens
}
