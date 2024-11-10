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
            let empty_line_literal = literals::EMPTY_LINE;
            if let Tokens::EmptyLine(empty_line) = empty_line_literal {
                assert_eq!(empty_line, "");
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
                            let heading_one_literal = literals::HEADING_ONE;
                            if let Tokens::HeadingOne(heading_one) = heading_one_literal {
                                assert_eq!(heading_one, format!("{} ", &"#".repeat(heading_level)));
                            }
                            Tokens::HeadingOne("# ")
                        }
                        2 => {
                            let heading_two_literal = literals::HEADING_TWO;
                            if let Tokens::HeadingTwo(heading_two) = heading_two_literal {
                                assert_eq!(heading_two, format!("{} ", &"#".repeat(heading_level)));
                            }
                            Tokens::HeadingTwo("## ")
                        }
                        3 => {
                            let heading_three_literal = literals::HEADING_THREE;
                            if let Tokens::HeadingThree(heading_three) = heading_three_literal {
                                assert_eq!(
                                    heading_three,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingThree("### ")
                        }
                        4 => {
                            let heading_four_literal = literals::HEADING_FOUR;
                            if let Tokens::HeadingFour(heading_four) = heading_four_literal {
                                assert_eq!(
                                    heading_four,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingFour("#### ")
                        }
                        5 => {
                            let heading_five_literal = literals::HEADING_FIVE;
                            if let Tokens::HeadingFive(heading_five) = heading_five_literal {
                                assert_eq!(
                                    heading_five,
                                    format!("{} ", &"#".repeat(heading_level))
                                );
                            }
                            Tokens::HeadingFive("##### ")
                        }
                        6 => {
                            let heading_six_literal = literals::HEADING_SIX;
                            if let Tokens::HeadingSix(heading_six) = heading_six_literal {
                                assert_eq!(heading_six, format!("{} ", &"#".repeat(heading_level)));
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
                let mut horizontal_rule_hyphen_level = 0;
                while horizontal_rule_hyphen_level < line.len()
                    && line.chars().nth(horizontal_rule_hyphen_level) == Some('-')
                {
                    horizontal_rule_hyphen_level += 1;
                }

                if horizontal_rule_hyphen_level == 3 && line.len() == 3 {
                    let horizontal_rule_hyphen_literal = literals::HORIZONTAL_RULE_HYPHEN;
                    if let Tokens::HorizontalRuleHyphen(horizontal_rule_hyphen) =
                        horizontal_rule_hyphen_literal
                    {
                        assert_eq!(horizontal_rule_hyphen, "---");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_hyphen".to_string(),
                        kind: Tokens::HorizontalRuleHyphen("---"),
                        value: format!("---"),
                    });
                } else {
                    let unordered_list_hyphen_literal = literals::UNORDERED_LIST_HYPHEN;
                    if let Tokens::UnorderedListHyphen(unordered_list_hyphen) =
                        unordered_list_hyphen_literal
                    {
                        assert_eq!(unordered_list_hyphen, "- ");
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
                    let horizontal_rule_asterisk_literal = literals::HORIZONTAL_RULE_ASTERISK;
                    if let Tokens::HorizontalRuleAsterisk(horizontal_rule_asterisk) =
                        horizontal_rule_asterisk_literal
                    {
                        assert_eq!(horizontal_rule_asterisk, "***");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_asterisk".to_string(),
                        kind: Tokens::HorizontalRuleAsterisk("***"),
                        value: format!("***"),
                    });
                }
            }
            '_' => {
                let mut horizontal_rule_underscore_level = 0;
                while horizontal_rule_underscore_level < line.len()
                    && line.chars().nth(horizontal_rule_underscore_level) == Some('_')
                {
                    horizontal_rule_underscore_level += 1;
                }
                if horizontal_rule_underscore_level == 3 && line.len() == 3 {
                    let horizontal_rule_underscore_literal = literals::HORIZONTAL_RULE_UNDERSCORE;
                    if let Tokens::HorizontalRuleUnderscore(horizontal_rule_underscore) =
                        horizontal_rule_underscore_literal
                    {
                        assert_eq!(horizontal_rule_underscore, "___");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_underscore".to_string(),
                        kind: Tokens::HorizontalRuleUnderscore("___"),
                        value: format!("___"),
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
