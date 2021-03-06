use std::{hash::Hash, mem::replace};

use lrlex::LRNonStreamingLexerDef;
use lrpar::{LexParseError, Lexeme, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};
use try_from::TryFrom;

use crate::internals::parser::ast::statement::Body;
use crate::internals::parser::span::{Span, Spanner};
use crate::internals::parser::traits::SyntaxError;

pub mod lexer;
pub use self::lexer::lexer_l::lexerdef;
pub mod parser;
pub use self::parser::parser_y::{parse, token_epp};

/// master function for parsing source code
#[allow(dead_code)]
pub fn parse_code<E>(source: &str) -> Result<Body, Vec<E>>
where
    E: SyntaxError,
{
    let def: LRNonStreamingLexerDef<u32> = lexerdef();
    {
        let def2 = def.lexer(source);
        parse_source(source, &def2, &parse)
    }
}

/// master function for serializing source code
#[allow(dead_code)]
pub fn serialize_ast(source: &Body) -> Result<String, String> {
    match serde_json::to_string_pretty(source) {
        Ok(arg) => Ok(arg),
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// master deserialize function
#[allow(dead_code)]
pub fn deserialize_ast(source: &str) -> Result<Body, String> {
    match serde_json::from_str::<Body>(source) {
        Ok(arg) => Ok(arg),
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// parse_source is a generic error handling function
/// its type arguments can be adjusted to finalize the
/// output.
fn parse_source<'lexer, 'input, U, P, E>(
    source: &'input str,
    def2: &'lexer dyn NonStreamingLexer<'input, U>,
    parser: &P,
) -> Result<Body, Vec<E>>
where
    'input: 'lexer,
    E: SyntaxError,
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
    P: Fn(
        &'lexer dyn NonStreamingLexer<'input, U>,
    ) -> (Option<Result<Body, Lexeme<U>>>, Vec<LexParseError<U>>),
{
    let (output, errors) = parser(def2);

    // iterate over parsing errors (if any occured)
    let mut errors_out: Vec<E> = errors
        .into_iter()
        .map(|err| match err {
            LexParseError::LexError(err) => {
                let span = Span::new_panic(def2, err.span());
                E::lex_error(def2, source, &span)
            }
            LexParseError::ParseError(parse_err) => {
                let span = Span::new_panic(def2, parse_err.lexeme());
                E::parse_error(def2, source, &span)
            }
        })
        .collect();

    // extract a result
    let mut return_value: Option<Body> = None;
    match output {
        Option::Some(Err(lex)) => {
            let span = Span::new_panic(def2, lex);
            errors_out.push(E::parse_error(def2, source, &span));
        }
        Option::Some(Ok(result)) => {
            return_value = Some(result);
        }
        Option::None => {
            // nothing to do
        }
    };

    // determine final result
    if errors_out.len() > 0 {
        Err(errors_out)
    } else if return_value.is_some() {
        Ok(return_value.unwrap())
    } else {
        panic!("impossible condition. Nothing was parsed, but no errors occured");
    }
}

#[cfg(test)]
mod test {
    use super::{lexerdef, parse};
    use crate::internals::parser::ast::statement::Body;

    const SAMPLES: &'static [&'static str] = &[
        include_str!("samples/example1.fx"),
        include_str!("samples/example2.fx"),
    ];

    fn parse_text<'a>(text: &'a str) -> Option<Body> {
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
