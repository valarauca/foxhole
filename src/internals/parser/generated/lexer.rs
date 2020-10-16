pub mod lexer_l {
    use lrlex::{LRNonStreamingLexerDef, LexerDef, Rule};

    #[allow(dead_code)]
    pub fn lexerdef() -> LRNonStreamingLexerDef<u32> {
        let rules = vec![
            Rule::new(Some(26), Some("NUM".to_string()), "[0-9]+".to_string()).unwrap(),
            Rule::new(None, Some("ANALYZE".to_string()), "analyze".to_string()).unwrap(),
            Rule::new(Some(25), Some("COMP".to_string()), "cm".to_string()).unwrap(),
            Rule::new(Some(24), Some("FN".to_string()), "fn".to_string()).unwrap(),
            Rule::new(Some(33), Some("INT".to_string()), "int".to_string()).unwrap(),
            Rule::new(Some(34), Some("BOOL".to_string()), "bool".to_string()).unwrap(),
            Rule::new(Some(27), Some("TRUE".to_string()), "true".to_string()).unwrap(),
            Rule::new(Some(28), Some("FALSE".to_string()), "false".to_string()).unwrap(),
            Rule::new(
                Some(35),
                Some("VEC_INT".to_string()),
                "vec<int>".to_string(),
            )
            .unwrap(),
            Rule::new(
                Some(36),
                Some("VEC_BOOL".to_string()),
                "vec<bool>".to_string(),
            )
            .unwrap(),
            Rule::new(Some(20), Some("LET".to_string()), "let".to_string()).unwrap(),
            Rule::new(Some(16), Some("IF".to_string()), "if".to_string()).unwrap(),
            Rule::new(Some(19), Some("ELSE".to_string()), "else".to_string()).unwrap(),
            Rule::new(
                Some(32),
                Some("TEMPLATE_FALLBACK".to_string()),
                ":-".to_string(),
            )
            .unwrap(),
            Rule::new(
                Some(31),
                Some("TEMPLATE_ASSIGN".to_string()),
                ":=".to_string(),
            )
            .unwrap(),
            Rule::new(
                Some(30),
                Some("TEMPLATE_START".to_string()),
                "\\$\\{".to_string(),
            )
            .unwrap(),
            Rule::new(Some(22), Some("ASSIGN".to_string()), "=".to_string()).unwrap(),
            Rule::new(Some(17), Some("RBRACE".to_string()), "\\{".to_string()).unwrap(),
            Rule::new(Some(18), Some("LBRACE".to_string()), "\\}".to_string()).unwrap(),
            Rule::new(Some(14), Some("LPAR".to_string()), "\\(".to_string()).unwrap(),
            Rule::new(Some(15), Some("RPAR".to_string()), "\\)".to_string()).unwrap(),
            Rule::new(Some(13), Some("XOR".to_string()), "\\^".to_string()).unwrap(),
            Rule::new(Some(12), Some("OR".to_string()), "\\|".to_string()).unwrap(),
            Rule::new(Some(11), Some("AND".to_string()), "&".to_string()).unwrap(),
            Rule::new(Some(23), Some("COMMA".to_string()), ",".to_string()).unwrap(),
            Rule::new(Some(0), Some("SEMI".to_string()), ";".to_string()).unwrap(),
            Rule::new(Some(21), Some("COLON".to_string()), ":".to_string()).unwrap(),
            Rule::new(Some(1), Some("ADD".to_string()), "\\+".to_string()).unwrap(),
            Rule::new(Some(3), Some("MUL".to_string()), "\\*".to_string()).unwrap(),
            Rule::new(Some(2), Some("SUB".to_string()), "-".to_string()).unwrap(),
            Rule::new(Some(4), Some("DIV".to_string()), "/".to_string()).unwrap(),
            Rule::new(Some(5), Some("EQ".to_string()), "==".to_string()).unwrap(),
            Rule::new(Some(6), Some("NE".to_string()), "!=".to_string()).unwrap(),
            Rule::new(Some(8), Some("LT".to_string()), "<".to_string()).unwrap(),
            Rule::new(Some(7), Some("GT".to_string()), ">".to_string()).unwrap(),
            Rule::new(Some(10), Some("LE".to_string()), "<=".to_string()).unwrap(),
            Rule::new(Some(9), Some("GE".to_string()), ">=".to_string()).unwrap(),
            Rule::new(
                Some(29),
                Some("IDENT".to_string()),
                "[a-zA-Z_][a-zA-Z0-9_]*".to_string(),
            )
            .unwrap(),
            Rule::new(
                Some(38),
                None,
                "((//|#![^\\n\\r]*)|[ \\t\\n\\r]+)".to_string(),
            )
            .unwrap(),
        ];
        LRNonStreamingLexerDef::from_rules(rules)
    }
    #[allow(dead_code)]
    pub const T_VEC_BOOL: u32 = 36;
    #[allow(dead_code)]
    pub const T_LPAR: u32 = 14;
    #[allow(dead_code)]
    pub const T_GE: u32 = 9;
    #[allow(dead_code)]
    pub const T_ADD: u32 = 1;
    #[allow(dead_code)]
    pub const T_FN: u32 = 24;
    #[allow(dead_code)]
    pub const T_FALSE: u32 = 28;
    #[allow(dead_code)]
    pub const T_TRUE: u32 = 27;
    #[allow(dead_code)]
    pub const T_COMMA: u32 = 23;
    #[allow(dead_code)]
    pub const T_TEMPLATE_START: u32 = 30;
    #[allow(dead_code)]
    pub const T_LET: u32 = 20;
    #[allow(dead_code)]
    pub const T_MUL: u32 = 3;
    #[allow(dead_code)]
    pub const T_OR: u32 = 12;
    #[allow(dead_code)]
    pub const T_XOR: u32 = 13;
    #[allow(dead_code)]
    pub const T_ELSE: u32 = 19;
    #[allow(dead_code)]
    pub const T_LT: u32 = 8;
    #[allow(dead_code)]
    pub const T_LBRACE: u32 = 18;
    #[allow(dead_code)]
    pub const T_VEC_INT: u32 = 35;
    #[allow(dead_code)]
    pub const T_SUB: u32 = 2;
    #[allow(dead_code)]
    pub const T_TEMPLATE_FALLBACK: u32 = 32;
    #[allow(dead_code)]
    pub const T_NUM: u32 = 26;
    #[allow(dead_code)]
    pub const T_INT: u32 = 33;
    #[allow(dead_code)]
    pub const T_DIV: u32 = 4;
    #[allow(dead_code)]
    pub const T_NE: u32 = 6;
    #[allow(dead_code)]
    pub const T_IDENT: u32 = 29;
    #[allow(dead_code)]
    pub const T_GT: u32 = 7;
    #[allow(dead_code)]
    pub const T_IF: u32 = 16;
    #[allow(dead_code)]
    pub const T_LE: u32 = 10;
    #[allow(dead_code)]
    pub const T_AND: u32 = 11;
    #[allow(dead_code)]
    pub const T_COLON: u32 = 21;
    #[allow(dead_code)]
    pub const T_COMP: u32 = 25;
    #[allow(dead_code)]
    pub const T_EQ: u32 = 5;
    #[allow(dead_code)]
    pub const T_BOOL: u32 = 34;
    #[allow(dead_code)]
    pub const T_RPAR: u32 = 15;
    #[allow(dead_code)]
    pub const T_SEMI: u32 = 0;
    #[allow(dead_code)]
    pub const T_ASSIGN: u32 = 22;
    #[allow(dead_code)]
    pub const T_TEMPLATE_ASSIGN: u32 = 31;
    #[allow(dead_code)]
    pub const T_RBRACE: u32 = 17;
}
