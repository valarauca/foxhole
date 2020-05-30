pub mod lexer;
pub use self::lexer::lexer_l::lexerdef;
pub mod parser;
pub use self::parser::parser_y::{parse, token_epp};

#[cfg(test)]
mod test {
    use super::{lexerdef, parse};
    use crate::internals::parser::ast::statement::Statement;

    const SAMPLES: &'static [&'static str] = &[
        include_str!("samples/example1.fx"),
        include_str!("samples/example2.fx"),
    ];

    fn parse_text<'a>(text: &'a str) -> Option<Vec<Statement>> {
        let def = lexerdef();
        {
            match parse(&def.lexer(text)) {
                (Option::Some(Ok(output)), _) => Some(output),
                _ => None,
            }
        }
    }

    #[test]
    fn parse_code() {
        for (i, sample) in SAMPLES.iter().enumerate() {
            match parse_text(sample) {
                Option::None => {
                    panic!("failed to parse sample:{}", i + 1);
                }
                Option::Some(_) => {}
            };
        }
    }
}
