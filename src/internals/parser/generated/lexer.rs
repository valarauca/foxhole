mod lexer_l {
use lrlex::{LexerDef, LRNonStreamingLexerDef, Rule};

#[allow(dead_code)]
pub fn lexerdef() -> LRNonStreamingLexerDef<u32> {
    let rules = vec![
Rule::new(Some(0), Some("NUM".to_string()), "[0-9]+".to_string()).unwrap(),
Rule::new(None, Some("ANALYZE".to_string()), "analyze".to_string()).unwrap(),
Rule::new(Some(1), Some("LET".to_string()), "let".to_string()).unwrap(),
Rule::new(None, Some("COMMENT".to_string()), "(//|#!)[^\\n\\r]*".to_string()).unwrap(),
Rule::new(Some(10), Some("TEMPLATE_FALLBACK".to_string()), ":-".to_string()).unwrap(),
Rule::new(Some(8), Some("TEMPLATE_ASSIGN".to_string()), ":=".to_string()).unwrap(),
Rule::new(Some(7), Some("TEMPLATE_START".to_string()), "\\$\\{".to_string()).unwrap(),
Rule::new(Some(9), Some("RBRACE".to_string()), "\\{".to_string()).unwrap(),
Rule::new(None, Some("LBRACE".to_string()), "\\}".to_string()).unwrap(),
Rule::new(Some(3), Some("LPAR".to_string()), "\\(".to_string()).unwrap(),
Rule::new(Some(4), Some("RPAR".to_string()), "\\)".to_string()).unwrap(),
Rule::new(None, Some("XOR".to_string()), "\\^".to_string()).unwrap(),
Rule::new(None, Some("OR".to_string()), "\\|".to_string()).unwrap(),
Rule::new(None, Some("AND".to_string()), "&".to_string()).unwrap(),
Rule::new(Some(5), Some("COMMA".to_string()), ",".to_string()).unwrap(),
Rule::new(None, Some("SEMI".to_string()), ";".to_string()).unwrap(),
Rule::new(None, Some("COLON".to_string()), ":".to_string()).unwrap(),
Rule::new(None, Some("ADD".to_string()), "\\+".to_string()).unwrap(),
Rule::new(None, Some("MUL".to_string()), "\\*".to_string()).unwrap(),
Rule::new(None, Some("SUB".to_string()), "-".to_string()).unwrap(),
Rule::new(None, Some("DIV".to_string()), "/".to_string()).unwrap(),
Rule::new(Some(2), Some("EQ".to_string()), "==".to_string()).unwrap(),
Rule::new(None, Some("LT".to_string()), "<".to_string()).unwrap(),
Rule::new(None, Some("GT".to_string()), ">".to_string()).unwrap(),
Rule::new(None, Some("LE".to_string()), "<=".to_string()).unwrap(),
Rule::new(None, Some("GE".to_string()), ">=".to_string()).unwrap(),
Rule::new(Some(6), Some("IDENT".to_string()), "[a-zA-Z_][a-zA-Z0-9_]*".to_string()).unwrap(),
Rule::new(Some(27), None, "[ \\t\\n\\r]+".to_string()).unwrap(),
];
    LRNonStreamingLexerDef::from_rules(rules)
}
#[allow(dead_code)]
pub const T_EQ: u32 = 2;
#[allow(dead_code)]
pub const T_IDENT: u32 = 6;
#[allow(dead_code)]
pub const T_RPAR: u32 = 4;
#[allow(dead_code)]
pub const T_NUM: u32 = 0;
#[allow(dead_code)]
pub const T_COMMA: u32 = 5;
#[allow(dead_code)]
pub const T_RBRACE: u32 = 9;
#[allow(dead_code)]
pub const T_TEMPLATE_FALLBACK: u32 = 10;
#[allow(dead_code)]
pub const T_LPAR: u32 = 3;
#[allow(dead_code)]
pub const T_TEMPLATE_ASSIGN: u32 = 8;
#[allow(dead_code)]
pub const T_LET: u32 = 1;
#[allow(dead_code)]
pub const T_TEMPLATE_START: u32 = 7;
}