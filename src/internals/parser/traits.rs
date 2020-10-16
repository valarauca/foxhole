use std::fmt::Display;
use std::hash::Hash;

use lrpar::{Lexer, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};

use crate::internals::parser::span::Span;

/// SyntaxError is a general interface which can used to iterate upon how
/// syntax errors are handled. The idea is if we need to change/adapt error
/// reporting in the future, the new "reporter" only needs to implement this
/// trait.
pub trait SyntaxError: Sized + Display {
    /// A lexer error occured
    fn lex_error<'a, U, L>(lexer: &L, source: &'a str, span: &Span) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'a, U> + Lexer<U> + ?Sized;

    /// Parse error occured -within- a parser
    fn parse_error<'a, U, L>(lexer: &L, source: &'a str, span: &Span) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'a, U> + Lexer<U> + ?Sized;
}
