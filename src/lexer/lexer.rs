use crate::elements::{literals, tokens::Tokens};

pub struct Token {
    line_number: usize,
    name: String,
    kind: Tokens,
    value: String,
}

pub fn parse(line_number: usize, line: &str) {
    let mut tokens = Vec::new();
    lex(line_number, line, &mut tokens);

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

pub fn lex(line_number: usize, line: &str, tokens: &mut Vec<Token>) {
    match line.chars().nth(0) {
        None => {
            let empty_line = literals::EMPTY_LINE;
            if let Tokens::EmptyLine(empty_line_literal) = empty_line {
                assert_eq!(empty_line_literal, "");
            }

            tokens.push(Token {
                line_number,
                name: "empty_line".to_string(),
                kind: Tokens::EmptyLine(""),
                value: "".to_string(),
            });
        }
        Some(c) => match c {
            '#' => {
                let mut heading_level = 1;

                while heading_level < line.len() && line.chars().nth(heading_level) == Some('#') {
                    heading_level += 1;
                }

                if heading_level <= 6 {
                    let token_kind = match heading_level {
                        1 => {
                            let heading_one = literals::HEADING_ONE;
                            if let Tokens::HeadingOne(heading_one_literal) = heading_one {
                                assert_eq!(
                                    heading_one_literal,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingOne("# ")
                        }
                        2 => {
                            let heading_two = literals::HEADING_TWO;
                            if let Tokens::HeadingTwo(heading_two_literal) = heading_two {
                                assert_eq!(
                                    heading_two_literal,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingTwo("## ")
                        }
                        3 => {
                            let heading_three = literals::HEADING_THREE;
                            if let Tokens::HeadingThree(heading_three_literal) = heading_three {
                                assert_eq!(
                                    heading_three_literal,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingThree("### ")
                        }
                        4 => {
                            let heading_four = literals::HEADING_FOUR;
                            if let Tokens::HeadingFour(heading_four_literal) = heading_four {
                                assert_eq!(
                                    heading_four_literal,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingFour("#### ")
                        }
                        5 => {
                            let heading_five = literals::HEADING_FIVE;
                            if let Tokens::HeadingFive(heading_five_literal) = heading_five {
                                assert_eq!(
                                    heading_five_literal,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingFive("##### ")
                        }
                        6 => {
                            let heading_six = literals::HEADING_SIX;
                            if let Tokens::HeadingSix(heading_six_literal) = heading_six {
                                assert_eq!(
                                    heading_six_literal,
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
                        line_number,
                        name: format!("h{}", heading_level),
                        kind: token_kind,
                        value: format!("{} {}", "#".repeat(heading_level), heading_text),
                    });
                }
            }
            '-' => {
                let hyphen_index = 0;
                let mut horizontal_rule_hyphen_level = 0;
                while hyphen_index < line.len()
                    && line
                        .chars()
                        .nth(hyphen_index + horizontal_rule_hyphen_level)
                        == Some('-')
                {
                    horizontal_rule_hyphen_level += 1;
                }

                if horizontal_rule_hyphen_level == 3 && line.len() == 3 {
                    let horizontal_rule_hyphen = literals::HORIZONTAL_RULE_HYPHEN;
                    if let Tokens::HorizontalRuleHyphen(horizontal_rule_hyphen_literal) =
                        horizontal_rule_hyphen
                    {
                        assert_eq!(horizontal_rule_hyphen_literal, "---");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_hyphen".to_string(),
                        kind: Tokens::HorizontalRuleHyphen("---"),
                        value: format!("---"),
                    });
                } else {
                    let unordered_list_hyphen = literals::UNORDERED_LIST_HYPHEN;
                    if let Tokens::UnorderedListHyphen(unordered_list_hyphen_literal) =
                        unordered_list_hyphen
                    {
                        assert_eq!(unordered_list_hyphen_literal, "- ");
                    }

                    let line_text = String::from(line);
                    let unordered_list_text = &line_text[2..line_text.len()];

                    tokens.push(Token {
                        line_number,
                        name: "unordered_list_hyphen".to_string(),
                        kind: Tokens::UnorderedListHyphen("- "),
                        value: format!("{}{}", "- ".to_string(), unordered_list_text),
                    });
                }
            }
            '*' => {
                let mut horizontal_rule_asterisk_level = 0;
                while horizontal_rule_asterisk_level < line.len()
                    && line.chars().nth(horizontal_rule_asterisk_level) == Some('*')
                {
                    horizontal_rule_asterisk_level += 1;
                }

                if horizontal_rule_asterisk_level == 3 && line.len() == 3 {
                    let horizontal_rule_asterisk = literals::HORIZONTAL_RULE_ASTERISK;
                    if let Tokens::HorizontalRuleAsterisk(horizontal_rule_asterisk_literal) =
                        horizontal_rule_asterisk
                    {
                        assert_eq!(horizontal_rule_asterisk_literal, "***");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_asterisk".to_string(),
                        kind: Tokens::HorizontalRuleAsterisk("***"),
                        value: format!("***"),
                    });
                }
            }
            _ => {
                tokens.push(Token {
                    line_number,
                    name: "text".to_string(),
                    kind: Tokens::Text(String::from(line)),
                    value: String::from(line),
                });
            }
        },
    }
}
