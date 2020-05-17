
use crate::internals::parser::{
    span::{Span,Spanner},
    ast::ident::{Ident},
};

/// Template is a variable who's value is given at run time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Template<'input> {
    pub span: Span<'input>,
    pub ident: Ident<'input>,
    pub behavior: Option<TemplateBehavior<'input>>,
}
impl<'input> Template<'input> {
    /// build a new template
    pub fn new<B>(ident: Ident<'input>, span: Span<'input>, behavior: B) -> Template<'input>
    where
        B: Into<Option<TemplateBehavior<'input>>>,
    {
        let behavior = behavior.into();
        Template {
            span,
            ident,
            behavior,
        }
    }
}
impl<'input> AsRef<Span<'input>> for Template<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Template<'input> {}

/// TemplateBehavior defines fallback behavior
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TemplateBehavior<'input> {
    /// Fallback will just assign the value to what ever is contained in the fallback
    Fallback(TemplateFallback<'input>),

    /// Assign will modify the global environment to set this value.
    Assign(TemplateFallback<'input>),
}
impl<'input> TemplateBehavior<'input> {
    #[inline(always)]
    pub fn fallback<F>(arg: F) -> TemplateBehavior<'input>
    where
        TemplateFallback<'input>: From<F>,
    {
        TemplateBehavior::Fallback(TemplateFallback::from(arg))
    }

    #[inline(always)]
    pub fn assign<F>(arg: F) -> TemplateBehavior<'input>
    where
        TemplateFallback<'input>: From<F>,
    {
        TemplateBehavior::Assign(TemplateFallback::from(arg))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TemplateFallback<'input> {
    Num(Box<Span<'input>>),
    Template(Box<Template<'input>>),
}
impl<'input> From<Span<'input>> for TemplateFallback<'input> {
    fn from(s: Span<'input>) -> TemplateFallback<'input> {
        TemplateFallback::Num(Box::new(s))
    }
}
impl<'input> From<Template<'input>> for TemplateFallback<'input> {
    fn from(s: Template<'input>) -> TemplateFallback<'input> {
        TemplateFallback::Template(Box::new(s))
    }
}
