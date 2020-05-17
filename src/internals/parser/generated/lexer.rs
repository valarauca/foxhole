mod lexer_l {
use lrlex::{NonStreamingLexerDef, Rule};

#[allow(dead_code)]
pub fn lexerdef() -> NonStreamingLexerDef<u32> {
    let rules = vec![
Rule::new(Some(4), Some("NUM".to_string()), "[0-9]+".to_string()).unwrap(),
Rule::new(None, Some("ANALYZE".to_string()), "analyze".to_string()).unwrap(),
Rule::new(None, Some("LET".to_string()), "let".to_string()).unwrap(),
Rule::new(None, Some("COMMENT".to_string()), "(//|#!)[^\\n\\r]*".to_string()).unwrap(),
Rule::new(Some(5), Some("TEMPLATE_FALLBACK".to_string()), ":-".to_string()).unwrap(),
Rule::new(Some(2), Some("TEMPLATE_ASSIGN".to_string()), ":=".to_string()).unwrap(),
Rule::new(Some(1), Some("TEMPLATE_START".to_string()), "\\$\\{".to_string()).unwrap(),
Rule::new(Some(3), Some("RBRACE".to_string()), "\\{".to_string()).unwrap(),
Rule::new(None, Some("LBRACE".to_string()), "\\}".to_string()).unwrap(),
Rule::new(None, Some("LPAR".to_string()), "\\(".to_string()).unwrap(),
Rule::new(None, Some("RPAR".to_string()), "\\)".to_string()).unwrap(),
Rule::new(None, Some("XOR".to_string()), "\\^".to_string()).unwrap(),
Rule::new(None, Some("OR".to_string()), "\\|".to_string()).unwrap(),
Rule::new(None, Some("AND".to_string()), "&".to_string()).unwrap(),
Rule::new(None, Some("COMMA".to_string()), ",".to_string()).unwrap(),
Rule::new(None, Some("SEMI".to_string()), ";".to_string()).unwrap(),
Rule::new(None, Some("COLON".to_string()), ":".to_string()).unwrap(),
Rule::new(None, Some("ADD".to_string()), "\\+".to_string()).unwrap(),
Rule::new(None, Some("MUL".to_string()), "\\*".to_string()).unwrap(),
Rule::new(None, Some("SUB".to_string()), "-".to_string()).unwrap(),
Rule::new(None, Some("DIV".to_string()), "/".to_string()).unwrap(),
Rule::new(None, Some("EQ".to_string()), "==".to_string()).unwrap(),
Rule::new(None, Some("LT".to_string()), "<".to_string()).unwrap(),
Rule::new(None, Some("GT".to_string()), ">".to_string()).unwrap(),
Rule::new(None, Some("LE".to_string()), "<=".to_string()).unwrap(),
Rule::new(None, Some("GE".to_string()), ">=".to_string()).unwrap(),
Rule::new(Some(0), Some("IDENT".to_string()), "[a-zA-Z_][a-zA-Z0-9_]*".to_string()).unwrap(),
Rule::new(Some(27), None, "[ \\t\\n\\r]+".to_string()).unwrap(),
];
    NonStreamingLexerDef::new(rules)
}
#[allow(dead_code)]
pub const T_TEMPLATE_ASSIGN: u32 = 2;
#[allow(dead_code)]
pub const T_TEMPLATE_START: u32 = 1;
#[allow(dead_code)]
pub const T_TEMPLATE_FALLBACK: u32 = 5;
#[allow(dead_code)]
pub const T_NUM: u32 = 4;
#[allow(dead_code)]
pub const T_IDENT: u32 = 0;
#[allow(dead_code)]
pub const T_RBRACE: u32 = 3;
}