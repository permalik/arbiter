use crate::elements::tokens::Tokens;

pub const HEADING_ONE: Tokens = Tokens::HeadingOne("# ");
pub const HEADING_TWO: Tokens = Tokens::HeadingTwo("## ");
pub const HEADING_THREE: Tokens = Tokens::HeadingOne("### ");
pub const HEADING_FOUR: Tokens = Tokens::HeadingOne("#### ");
pub const HEADING_FIVE: Tokens = Tokens::HeadingTwo("##### ");
pub const HEADING_SIX: Tokens = Tokens::HeadingTwo("###### ");
pub const UNORDERED_LIST_HYPHEN: Tokens = Tokens::UnorderedListHyphen("- ");
pub const HORIZONTAL_RULE_HYPHEN: Tokens = Tokens::HorizontalRuleHyphen("---");
pub const HORIZONTAL_RULE_ASTERISK: Tokens = Tokens::HorizontalRuleAsterisk("***");
pub const HORIZONTAL_RULE_UNDERSCORE: Tokens = Tokens::HorizontalRuleUnderscore("___");
pub const BLOCKQUOTE: Tokens = Tokens::Blockquote("> ");
pub const EMPTY_LINE: Tokens = Tokens::EmptyLine("");
//pub const LINE_BREAK: Tokens = Tokens::LineBreak("  \n");
