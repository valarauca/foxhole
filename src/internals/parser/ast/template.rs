use crate::internals::parser::{
    ast::ident::Ident,
    span::{Span, Spanner},
};

use serde::{Deserialize, Serialize};

/// Template is a variable who's value is given at run time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Template {
    pub span: Box<Span>,

    pub ident: Box<Ident>,

    pub behavior: Option<TemplateBehavior>,
}

impl Template {
    /// build a new template
    pub fn new<B>(ident: Ident, span: Span, behavior: B) -> Template
    where
        B: Into<Option<TemplateBehavior>>,
    {
        let behavior = behavior.into();
        let ident = Box::new(ident);
        let span = Box::new(span);
        Template {
            span,
            ident,
            behavior,
        }
    }
}
impl AsRef<Span> for Template {
    #[inline(always)]
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Template {
    fn fields<'a>(&'a self) {
        self.set_id();
        self.ident.fields();
        match &self.behavior {
            &Option::None => {}
            &Option::Some(TemplateBehavior::Fallback(ref x))
            | &Option::Some(TemplateBehavior::Assign(ref x)) => {
                match x {
                    TemplateFallback::Num(ref span) => {
                        span.set_id();
                    }
                    TemplateFallback::Template(ref temp) => {
                        temp.fields();
                    }
                };
            }
        };
    }
}

/// TemplateBehavior defines fallback behavior
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TemplateBehavior {
    /// Fallback will just assign the value to what ever is contained in the fallback
    Fallback(TemplateFallback),

    /// Assign will modify the global environment to set this value.
    Assign(TemplateFallback),
}

impl TemplateBehavior {
    #[inline(always)]
    pub fn fallback<F>(arg: F) -> TemplateBehavior
    where
        TemplateFallback: From<F>,
    {
        TemplateBehavior::Fallback(TemplateFallback::from(arg))
    }

    #[inline(always)]
    pub fn assign<F>(arg: F) -> TemplateBehavior
    where
        TemplateFallback: From<F>,
    {
        TemplateBehavior::Assign(TemplateFallback::from(arg))
    }
}

/// TemplateFallback describes the fallback behavior, this type may not exist in all
/// circumstances as not all templates have fallback behavior.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TemplateFallback {
    Num(Box<Span>),

    Template(Box<Template>),
}

impl From<Span> for TemplateFallback {
    fn from(s: Span) -> TemplateFallback {
        TemplateFallback::Num(Box::new(s))
    }
}

impl From<Template> for TemplateFallback {
    fn from(s: Template) -> TemplateFallback {
        TemplateFallback::Template(Box::new(s))
    }
}
