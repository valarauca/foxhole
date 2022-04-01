use std::{borrow::ToOwned, hash::Hash, str::from_utf8_unchecked};

use serde::{Deserialize, Serialize};

// cargo doesn't realize I'm using this in a function signature.
#[allow(unused_imports)]
use lrpar::{Lexeme, Lexer, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};
use try_from::TryFrom;

/// Span contains information about where some text lies within the pre-parse structure
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    // the source code line the span starts on
    start_line: u32,
    // the source code line the span ends on
    end_line: u32,
    // the column the token starts on
    start_column: u32,
    // the column the token ends on
    end_column: u32,
    // the byte index of the source this token starts on
    start_byte: u32,
    // the byte index of the source this token ends on
    end_byte: u32,
    // how many bytes of 'surrounding_lines' precedes 'token'
    prefix_length: u32,
    // how many bytes of 'surrounding_lines' follows 'token'
    suffix_length: u32,
    // the parsed text itself
    token: String,
    // the line(s) (if it spans multiple lines) that contain this value.
    surrounding_lines: String,
}

impl Span {
    /// Build a Span.
    ///
    /// Ensure `s` and `span` are never both `None`. As this will trigger a panic.
    pub(in crate::internals::parser) fn new<'a, 'input: 'a, U, G, S, L>(
        l: &'a L,
        s: G,
        span: S,
    ) -> Result<Span, lrpar::Lexeme<U>>
    where
        L: NonStreamingLexer<'input, U> + Lexer<U> + ?Sized,
        U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
        S: Into<Option<lrpar::Span>>,
        G: Into<Option<Result<lrpar::Lexeme<U>, lrpar::Lexeme<U>>>>,
    {
        let span = match (span.into(), s.into()) {
            (Option::Some(s), Option::Some(Result::Ok(_))) | (Option::Some(s), Option::None) => s,
            (Option::Some(_), Option::Some(Result::Err(e)))
            | (Option::None, Option::Some(Result::Err(e))) => {
                return Err(e);
            }
            (Option::None, Option::Some(Result::Ok(x))) => x.span(),
            (Option::None, Option::None) => {
                panic!("Span::new() invoked without a Lexeme or Span");
            }
        };
        let token_borrowed = l.span_str(span.clone());
        let token_borrowed_start_address: usize = token_borrowed.as_ptr() as usize;
        let token_borrowed_end_address: usize = token_borrowed_start_address + token_borrowed.len();

        let surrounding_lines_borrowed = l.span_lines_str(span.clone());
        let surrounding_lines_borrowed_start_address: usize =
            surrounding_lines_borrowed.as_ptr() as usize;
        let surrounding_lines_borrowed_end_address: usize =
            surrounding_lines_borrowed_start_address + surrounding_lines_borrowed.len();

        let prefix_length = token_borrowed_start_address - surrounding_lines_borrowed_start_address;
        let suffix_length = surrounding_lines_borrowed_end_address - token_borrowed_end_address;

        debug_assert!(
            (prefix_length + suffix_length + token_borrowed.len())
                == surrounding_lines_borrowed.len()
        );

        let token = token_borrowed.to_owned();
        let surrounding_lines = surrounding_lines_borrowed.to_owned();
        let ((start_line, start_column), (end_line, end_column)) = l.line_col(span.clone());
        let start_byte = span.start();
        let end_byte = span.end();
        Ok(Span {
            start_line: start_line as u32,
            end_line: end_line as u32,
            start_column: start_column as u32,
            end_column: end_column as u32,
            start_byte: start_byte as u32,
            end_byte: end_byte as u32,
            prefix_length: prefix_length as u32,
            suffix_length: suffix_length as u32,
            token,
            surrounding_lines,
        })
    }

    /// creates a new span, but will not panic
    #[allow(dead_code)]
    pub(in crate::internals::parser) fn new_panic<'input, 'a, 'b, U, L, T>(l: &'a L, arg: T) -> Span
    where
        'a: 'b,
        'input: 'a,
        SpanBuilder<U>: From<T>,
        L: NonStreamingLexer<'input, U> + Lexer<U> + ?Sized,
        U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
    {
        let output = match SpanBuilder::from(arg) {
            SpanBuilder::Lexeme(lexeme) => Span::new(l, Ok(lexeme), None),
            SpanBuilder::Span(span) => Span::new(l, None, span),
        };
        match output {
            Ok(x) => x,
            _ => panic!(),
        }
    }

    pub(in crate::internals::parser) fn into<'input, 'a, 'b, U, S, L>(
        l: &'a L,
        span: S,
    ) -> impl 'b + FnOnce() -> Result<Span, lrpar::Lexeme<U>>
    where
        'a: 'b,
        'input: 'a,
        L: NonStreamingLexer<'input, U> + Lexer<U> + ?Sized,
        U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
        S: Into<Option<lrpar::Span>> + 'a,
    {
        move || Span::new(l, None, span)
    }
}
impl AsRef<Span> for Span {
    #[inline(always)]
    fn as_ref(&self) -> &Span {
        self
    }
}
impl Spanner for Span {}

/// SpanBuilder is used to handle the different things a "span" can be created from
pub enum SpanBuilder<U>
where
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
{
    Lexeme(lrpar::Lexeme<U>),
    Span(lrpar::Span),
}
impl<U> From<lrpar::Lexeme<U>> for SpanBuilder<U>
where
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
{
    fn from(arg: lrpar::Lexeme<U>) -> Self {
        Self::Lexeme(arg)
    }
}
impl<'a, U> From<&'a lrpar::Lexeme<U>> for SpanBuilder<U>
where
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
{
    fn from(arg: &'a lrpar::Lexeme<U>) -> Self {
        Self::Lexeme(arg.clone())
    }
}
impl<'a, U> From<&'a lrpar::Span> for SpanBuilder<U>
where
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
{
    fn from(arg: &'a lrpar::Span) -> Self {
        Self::Span(arg.clone())
    }
}
impl<U> From<lrpar::Span> for SpanBuilder<U>
where
    U: TryFrom<usize> + Eq + Copy + Unsigned + PrimInt + Hash + 'static,
{
    fn from(arg: lrpar::Span) -> Self {
        Self::Span(arg)
    }
}

/// Spanner is a trait which can be implemented on all AST Types.
///
/// It exists to simply the generation error messages as well as
/// getting source location more imply.
pub trait Spanner: AsRef<Span> {
    /// returns a copy of the span
    fn get_clone(&self) -> Span {
        self.as_ref().clone()
    }

    /// returns the byte index of the first byte of this span within the source file.
    fn get_start_byte_index(&self) -> usize {
        self.as_ref().start_byte as usize
    }

    /// returns the byte index of the last byte of this span within the source file.
    fn get_end_byte_index(&self) -> usize {
        self.as_ref().end_byte as usize
    }

    /// returns the line number that this span starts on.
    fn get_start_line(&self) -> usize {
        self.as_ref().start_line as usize
    }

    /// returns the line number that this span ends on.
    fn get_end_line(&self) -> usize {
        self.as_ref().end_line as usize
    }

    /// Is this span isolated to one line
    fn is_one_line(&self) -> bool {
        self.get_start_line() == self.get_end_line()
    }

    /// returns the column number (within the line) that this span starts on.
    fn get_start_column(&self) -> usize {
        self.as_ref().start_column as usize
    }

    /// returns the column number (within the line) that this span ends on.
    fn get_end_column(&self) -> usize {
        self.as_ref().end_column as usize
    }

    /// returns the underlying `str` representation of the input.
    fn get_span<'a>(&'a self) -> &'a str {
        &self.as_ref().token
    }

    /// returns how many bytes of `get_surrounding_lines` are a prefix to `token`
    fn get_prefix_length(&self) -> usize {
        self.as_ref().prefix_length as usize
    }

    /// returns how many bytes of `get_suffix_length` are a suffix to `token`
    fn get_suffix_length(&self) -> usize {
        self.as_ref().suffix_length as usize
    }

    /// returns the 3 parts of the span
    fn get_prefix_token_suffix<'a>(&'a self) -> (&'a str, &'a str, &'a str) {
        let chunk = self.get_surrounding_lines().as_bytes();
        // this is unsafe, but this information was given by the parser
        // so we'll implicitly trust it.
        let token = self.get_span();
        unsafe {
            let prefix = from_utf8_unchecked(&chunk[0..self.get_prefix_length()]);
            let skip = prefix.len() + token.len();
            let suffix = from_utf8_unchecked(&chunk[skip..]);
            (prefix, token, suffix)
        }
    }

    /// returns the raw line(s) (multiple if "this span" crosses multiple lines) which "this span"
    /// is contained within.
    fn get_surrounding_lines<'a>(&'a self) -> &'a str {
        &self.as_ref().surrounding_lines
    }
}
