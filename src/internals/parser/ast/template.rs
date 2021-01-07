use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{
        build_data_child_lambda, build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph,
        Node, NodeIndex, NodeTrait,
    },
    parser::{
        ast::ident::Ident,
        span::{Span, Spanner},
    },
};

/// Template is a variable who's value is given at run time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Template {
    pub span: Box<Span>,

    pub ident: Box<Ident>,

    pub behavior: Option<TemplateBehavior>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateSpan;

impl EdgeTrait for TemplateSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateIdent;

impl EdgeTrait for TemplateIdent {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateBehaviorEdge;

impl EdgeTrait for TemplateBehaviorEdge {
    type N = TemplateBehavior;
}

impl NodeTrait for Template {
    fn children(&self) -> Vec<ChildLambda> {
        let mut v = vec![
            build_typed_child_lambda::<_, TemplateSpan>(&self.span),
            build_typed_child_lambda::<_, TemplateIdent>(&self.ident),
        ];
        v.extend(
            self.behavior
                .clone()
                .into_iter()
                .map(|arg| build_data_child_lambda(&arg, TemplateBehaviorEdge::default())),
        );
        v
    }
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

impl Spanner for Template {}

/// TemplateBehavior defines fallback behavior
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TemplateBehavior {
    /// Fallback will just assign the value to what ever is contained in the fallback
    Fallback(TemplateFallback),

    /// Assign will modify the global environment to set this value.
    Assign(TemplateFallback),
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateBehaviorFallback;

impl EdgeTrait for TemplateBehaviorFallback {
    type N = TemplateFallback;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateBehaviorAssign;

impl EdgeTrait for TemplateBehaviorAssign {
    type N = TemplateFallback;
}

impl NodeTrait for TemplateBehavior {
    fn children(&self) -> Vec<ChildLambda> {
        vec![match self {
            &TemplateBehavior::Fallback(ref fallback) => {
                build_data_child_lambda(fallback, TemplateBehaviorFallback::default())
            }
            &TemplateBehavior::Assign(ref assign) => {
                build_data_child_lambda(assign, TemplateBehaviorAssign::default())
            }
        }]
    }
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

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FallbackNum;

impl EdgeTrait for FallbackNum {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FallbackTemplate;

impl EdgeTrait for FallbackTemplate {
    type N = Template;
}

impl NodeTrait for TemplateFallback {
    fn children(&self) -> Vec<ChildLambda> {
        let lambda: ChildLambda = match self {
            &TemplateFallback::Num(ref span) => {
                let span: Span = span.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(span);
                    graph.add_edge(parent, id, FallbackNum::default());
                })
            }
            &TemplateFallback::Template(ref template) => {
                let template: Template = template.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(template);
                    graph.add_edge(parent, id, FallbackTemplate::default());
                })
            }
        };
        vec![lambda]
    }
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
