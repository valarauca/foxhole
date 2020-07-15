use std::hash::Hash;

use serde::{Deserialize, Serialize};

// cargo doesn't realize I'm using this in a function signature.
#[allow(unused_imports)]
use lrpar::{Lexeme, Lexer, NonStreamingLexer};
use num_traits::{PrimInt, Unsigned};
use try_from::TryFrom;

use super::Id;

/// Span contains information about where some text lies within the pre-parse structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Span<'input> {
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
    // identifier is used to uniquely id this element.
    identifier: Id,
    // the parsed text itself
    #[serde(borrow)]
    token: &'input str,
    // the line(s) (if it spans multiple lines) that contain this value.
    #[serde(borrow)]
    surrounding_lines: &'input str,
}

impl<'input> PartialEq for Span<'input> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}
impl<'input> Eq for Span<'input> {}
impl<'input> PartialOrd for Span<'input> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.identifier.partial_cmp(&other.identifier)
    }
}
impl<'input> Ord for Span<'input> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}
impl<'input> std::hash::Hash for Span<'input> {
    #[inline]
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.identifier.hash(hasher);
    }
}

impl<'input> Span<'input> {
    /// Build a Span.
    ///
    /// Ensure `s` and `span` are never both `None`. As this will trigger a panic.
    pub(in crate::internals::parser) fn new<'a, U, G, S, L>(
        l: &'a L,
        s: G,
        span: S,
    ) -> Result<Span<'input>, lrpar::Lexeme<U>>
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
        let token = l.span_str(span.clone());
        let surrounding_lines = l.span_lines_str(span.clone());
        let ((start_line, start_column), (end_line, end_column)) = l.line_col(span.clone());
        let start_byte = span.start();
        let end_byte = span.end();
        let identifier = Id::default();
        Ok(Span {
            start_line: start_line as u32,
            end_line: end_line as u32,
            start_column: start_column as u32,
            end_column: end_column as u32,
            start_byte: start_byte as u32,
            end_byte: end_byte as u32,
            token,
            surrounding_lines,
            identifier,
        })
    }

    /// creates a new span, but will not panic
    #[allow(dead_code)]
    pub(in crate::internals::parser) fn new_panic<'a, 'b, U, L, T>(l: &'a L, arg: T) -> Span<'input>
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

    pub(in crate::internals::parser) fn into<'a, 'b, U, S, L>(
        l: &'a L,
        span: S,
    ) -> impl 'b + FnOnce() -> Result<Span<'input>, lrpar::Lexeme<U>>
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
impl<'input> AsRef<Span<'input>> for Span<'input> {
    #[inline(always)]
    fn as_ref(&self) -> &Span<'input> {
        self
    }
}
impl<'input> Spanner<'input> for Span<'input> {}

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
pub trait Spanner<'input>: AsRef<Span<'input>> {
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

    /// returns the column number (within the line) that this span starts on.
    fn get_start_column(&self) -> usize {
        self.as_ref().start_column as usize
    }

    /// returns the column number (within the line) that this span ends on.
    fn get_end_column(&self) -> usize {
        self.as_ref().end_column as usize
    }

    /// returns the underlying `str` representation of the input.
    fn get_span(&self) -> &'input str {
        self.as_ref().token
    }

    /// returns the raw line(s) (multiple if "this span" crosses multiple lines) which "this span"
    /// is contained within.
    fn get_surrounding_lines(&self) -> &'input str {
        self.as_ref().surrounding_lines
    }
}

#[test]
fn span_struct_is_a_cache_line() {
    use std::mem::size_of;
    assert_eq!(size_of::<Span>(), 64);
}
