use crate::elements::{literals, tokens::Tokens};

pub struct Token {
    name: String,
    kind: Tokens,
    value: String,
    position: usize,
}

pub fn parse(line: &str) {
    for token in lex(line) {
        println!(
            "Token: name: {:?}, kind: {:?}, value: {:?}, position: {:?}",
            token.name, token.kind, token.value, token.position
        );
        //println!(
        //    "Token: name: {:?}, kind: {:?}, value: {:?}, position: {:?}",
        //    token.name, token.kind, token.value, token.position
        //);
    }
}

pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < line.len() {
        if i == 0 {
            let c = line.chars().nth(i).unwrap();
            match c {
                '#' => {
                    let mut heading_level = 1;

                    while i + heading_level < line.len()
                        && line.chars().nth(i + heading_level) == Some('#')
                    {
                        heading_level += 1;
                    }

                    if heading_level <= 6 {
                        let token_kind = match heading_level {
                            1 => {
                                let heading_one = literals::HEADING_ONE;
                                if let Tokens::HeadingOne(heading_one_literal) = heading_one {
                                    assert_eq!(
                                        *heading_one_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingOne("# ")
                            }
                            2 => {
                                let heading_two = literals::HEADING_TWO;
                                if let Tokens::HeadingTwo(heading_two_literal) = heading_two {
                                    assert_eq!(
                                        *heading_two_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingTwo("## ")
                            }
                            3 => {
                                let heading_three = literals::HEADING_THREE;
                                if let Tokens::HeadingThree(heading_three_literal) = heading_three {
                                    assert_eq!(
                                        *heading_three_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingThree("### ")
                            }
                            4 => {
                                let heading_four = literals::HEADING_FOUR;
                                if let Tokens::HeadingFour(heading_four_literal) = heading_four {
                                    assert_eq!(
                                        *heading_four_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingFour("#### ")
                            }
                            5 => {
                                let heading_five = literals::HEADING_FIVE;
                                if let Tokens::HeadingFive(heading_five_literal) = heading_five {
                                    assert_eq!(
                                        *heading_five_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingFive("##### ")
                            }
                            6 => {
                                let heading_six = literals::HEADING_SIX;
                                if let Tokens::HeadingSix(heading_six_literal) = heading_six {
                                    assert_eq!(
                                        *heading_six_literal,
                                        format!("{} ", &"#".repeat(heading_level))
                                    );
                                }
                                Tokens::HeadingSix("###### ")
                            }
                            _ => unreachable!(),
                        };

                        let line_text = String::from(line);
                        let heading_text = &line_text[(heading_level + 1)..line_text.len()];

                        tokens.push(Token {
                            name: format!("h{}", heading_level),
                            kind: token_kind,
                            value: format!("{} {}", "#".repeat(heading_level), heading_text),
                            position: i,
                        });
                    }
                    i += heading_level;
                    continue;
                }
                '\n' => {
                    let mut is_line_break = false;
                    let mut line_break_position = 1;

                    while line_break_position < 3
                        && line.chars().nth(i + line_break_position) == Some(' ')
                    {
                        line_break_position += 1;
                        if line_break_position == 3 {
                            is_line_break = true;
                        }
                    }

                    if is_line_break {
                        let line_break = literals::LINE_BREAK;
                        if let Tokens::LineBreak(ref line_break_literal) = line_break {
                            assert_eq!(line_break_literal, &"  \n");
                            tokens.push({
                                Token {
                                    name: "line_break".to_string(),
                                    kind: Tokens::LineBreak("  \n"),
                                    value: line_break_literal.to_string(),
                                    position: i,
                                }
                            })
                        }
                    } else {
                        let newline = literals::NEWLINE;
                        if let Tokens::Newline(ref newline_literal) = newline {
                            assert_eq!(newline_literal, &"\n");
                            tokens.push({
                                Token {
                                    name: "newline".to_string(),
                                    kind: Tokens::Newline("\n"),
                                    value: newline_literal.to_string(),
                                    position: i,
                                }
                            });
                        }
                    }
                    i += 1;
                }
                ' ' => {
                    let space = literals::SPACE;
                    if let Tokens::Space(ref space_literal) = space {
                        assert_eq!(space_literal, &" ");
                        tokens.push({
                            Token {
                                name: "space".to_string(),
                                kind: Tokens::Space(" "),
                                value: space_literal.to_string(),
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
        } else {
            return tokens;
        }
    }
    tokens
}

//pub fn lex(file_contents: &str) -> Vec<Token> {
//    let mut tokens: Vec<Token> = Vec::new();
//    let mut i = 0;
//
//    while i < file_contents.len() {
//        assert!(i < file_contents.len(), "assert: index oob");
//        let c = file_contents.chars().nth(i).unwrap();
//        match c {
//            '#' => {
//                let mut heading_level = 1;
//
//                while i + heading_level < file_contents.len()
//                    && file_contents.chars().nth(i + heading_level) == Some('#')
//                {
//                    heading_level += 1;
//                }
//
//                if heading_level <= 6 {
//                    let (_heading_literal, token_kind) = match heading_level {
//                        1 => {
//                            let heading_one = literals::HEADING_ONE;
//                            if let Tokens::HeadingOne(ref heading_one_literal) = heading_one {
//                                assert_eq!(heading_one_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_one, Tokens::HeadingOne("#"))
//                        }
//                        2 => {
//                            let heading_two = literals::HEADING_TWO;
//                            if let Tokens::HeadingTwo(ref heading_two_literal) = heading_two {
//                                assert_eq!(heading_two_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_two, Tokens::HeadingTwo("##"))
//                        }
//                        3 => {
//                            let heading_three = literals::HEADING_THREE;
//                            if let Tokens::HeadingThree(ref heading_three_literal) = heading_three {
//                                assert_eq!(heading_three_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_three, Tokens::HeadingThree("###"))
//                        }
//                        4 => {
//                            let heading_four = literals::HEADING_FOUR;
//                            if let Tokens::HeadingFour(ref heading_four_literal) = heading_four {
//                                assert_eq!(heading_four_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_four, Tokens::HeadingFour("####"))
//                        }
//                        5 => {
//                            let heading_five = literals::HEADING_FIVE;
//                            if let Tokens::HeadingFive(ref heading_five_literal) = heading_five {
//                                assert_eq!(heading_five_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_five, Tokens::HeadingFive("#####"))
//                        }
//                        6 => {
//                            let heading_six = literals::HEADING_SIX;
//                            if let Tokens::HeadingSix(ref heading_six_literal) = heading_six {
//                                assert_eq!(heading_six_literal, &"#".repeat(heading_level));
//                            }
//                            (heading_six, Tokens::HeadingSix("######"))
//                        }
//                        _ => unreachable!(),
//                    };
//
//                    tokens.push(Token {
//                        kind: token_kind,
//                        value: format!("{}", "#".repeat(heading_level)),
//                        position: i,
//                    });
//                }
//                i += heading_level;
//                continue;
//            }
//            '\n' => {
//                let mut is_line_break = false;
//                let mut line_break_position = 1;
//
//                while line_break_position < 3
//                    && file_contents.chars().nth(i + line_break_position) == Some(' ')
//                {
//                    line_break_position += 1;
//                    if line_break_position == 3 {
//                        is_line_break = true;
//                    }
//                }
//
//                if is_line_break {
//                    let line_break = literals::LINE_BREAK;
//                    if let Tokens::LineBreak(ref line_break_literal) = line_break {
//                        assert_eq!(line_break_literal, &"  \n");
//                        tokens.push({
//                            Token {
//                                kind: Tokens::LineBreak("  \n"),
//                                value: line_break_literal.to_string(),
//                                position: i,
//                            }
//                        })
//                    }
//                } else {
//                    let newline = literals::NEWLINE;
//                    if let Tokens::Newline(ref newline_literal) = newline {
//                        assert_eq!(newline_literal, &"\n");
//                        tokens.push({
//                            Token {
//                                kind: Tokens::Newline("\n"),
//                                value: newline_literal.to_string(),
//                                position: i,
//                            }
//                        });
//                    }
//                }
//                i += 1;
//            }
//            ' ' => {
//                let space = literals::SPACE;
//                if let Tokens::Space(ref space_literal) = space {
//                    assert_eq!(space_literal, &" ");
//                    tokens.push({
//                        Token {
//                            kind: Tokens::Space(" "),
//                            value: space_literal.to_string(),
//                            position: i,
//                        }
//                    });
//                }
//                i += 1;
//            }
//            _ => {
//                i += 1;
//            }
//        }
//    }
//    assert!(
//        i > 0,
//        "assert: index i did not advance, potential infinite loop"
//    );
//    tokens
//}
