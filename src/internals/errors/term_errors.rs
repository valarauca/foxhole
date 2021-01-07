use std::{
    borrow::ToOwned,
    fmt,
    hash::Hash,
    iter::{once, repeat},
};

use ansi_term::{Color, Style};
use itertools::Itertools;
use lrpar::{Lexer, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};

use crate::internals::parser::{
    span::{Span, Spanner},
    traits::SyntaxError,
};

/// HumanReadable Errors
pub struct HumanReadable {
    arg: Vec<Item>,
}

impl fmt::Debug for HumanReadable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.arg.iter() {
            match item {
                Item::Owned(ref s) => f.write_str(s)?,
                Item::Static(s) => f.write_str(s)?,
            };
        }
        Ok(())
    }
}

impl fmt::Display for HumanReadable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <HumanReadable as fmt::Debug>::fmt(self, f)
    }
}

impl SyntaxError for HumanReadable {
    fn lex_error<'a, U, L>(_lexer: &L, _source: &'a str, span: &Span) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'a, U> + Lexer<U> + ?Sized,
    {
        Self {
            arg: vec![
                Item::from(Color::Red.normal().paint("ERROR")),
                Item::from(" Unrecongized Lex Item"),
            ]
            .into_iter()
            .chain(format_text(span).into_iter())
            .chain(once(Item::from(EOL)))
            .collect(),
        }
    }

    fn parse_error<'a, U, L>(_lexer: &L, _source: &'a str, span: &Span) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'a, U> + Lexer<U> + ?Sized,
    {
        Self {
            arg: vec![
                Item::from(Color::Red.normal().paint("ERROR")),
                Item::from(" Parse error"),
            ]
            .into_iter()
            .chain(format_text(span).into_iter())
            .chain(once(Item::from(EOL)))
            .collect(),
        }
    }
}

const EOL: &'static str = {
    #[cfg(target_family = "unix")]
    {
        "\n"
    }

    #[cfg(target_family = "windows")]
    {
        "\r\n"
    }
};

#[derive(Clone)]
enum Item {
    Owned(String),
    Static(&'static str),
}
impl Item {
    fn new(arg: &str) -> Item {
        Item::Owned(arg.to_owned())
    }
}
impl From<&'static str> for Item {
    fn from(arg: &'static str) -> Item {
        Item::Static(arg)
    }
}
impl From<String> for Item {
    fn from(arg: String) -> Item {
        Item::Owned(arg)
    }
}
impl<'a, S> From<ansi_term::ANSIGenericString<'a, S>> for Item
where
    S: 'a + ToOwned + ?Sized + fmt::Debug,
    <S as ToOwned>::Owned: fmt::Debug,
{
    fn from(arg: ansi_term::ANSIGenericString<'a, S>) -> Item {
        Item::Owned(format!("{:?}", arg))
    }
}
impl From<usize> for Item {
    fn from(arg: usize) -> Item {
        Item::Owned(format!("{:04} ", arg))
    }
}

fn format_text(span: &Span) -> Vec<Item> {
    let (prefix, token, suffix) = span.get_prefix_token_suffix();
    if span.is_one_line() {
        vec![
            Item::from(span.get_start_line()),
            Item::new(prefix),
            Item::from(Style::new().underline().paint(token)),
            Item::new(suffix),
        ]
    } else {
        (span.get_start_line()..(span.get_end_line() + 2))
            .map(|x| Item::from(x))
            .interleave_shortest(
                prefix
                    .lines()
                    .map(Item::new)
                    .chain(
                        token
                            .lines()
                            .map(|s| Item::from(Style::new().underline().paint(s))),
                    )
                    .chain(suffix.lines().map(Item::new))
                    .interleave_shortest(repeat(Item::from(EOL))),
            )
            .collect::<Vec<Item>>()
    }
}
