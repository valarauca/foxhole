use std::fmt::Display;
use std::hash::Hash;

use lrpar::{Lexer, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};

use crate::internals::parser::span::Span;

/// SyntaxError is a general interface which can used to iterate upon how
/// syntax errors are handled. The idea is if we need to change/adapt error
/// reporting in the future, the new "reporter" only needs to implement this
/// trait.
pub trait SyntaxError<'input>: Sized + Display {
    /// A lexer error occured
    fn lex_error<U, L>(lexer: &L, source: &'input str, span: &Span<'input>) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'input, U> + Lexer<U> + ?Sized;

    /// Parse error occured -within- a parser
    fn parse_error<U, L>(lexer: &L, source: &'input str, span: &Span<'input>) -> Self
    where
        U: Unsigned + PrimInt + Hash,
        L: NonStreamingLexer<'input, U> + Lexer<U> + ?Sized;
}
