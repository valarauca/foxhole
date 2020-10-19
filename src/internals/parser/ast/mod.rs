#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/*
 * Useful macro for handling enums
 *
 */
macro_rules! stuff {
    (
        Name: $Kind: ident;
        Trait: $TraitName: ident;
        From: { $($FromKind: ty => $Variant: ident => $is_a: ident => $get_a: ident),* $(,)*}) => {

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum $Kind {
            $(
                 $Variant(Box<$FromKind>),
            )*
        }

        impl AsRef<$Kind> for $Kind {
            fn as_ref<'a>(&'a self) -> &'a Self {
                self
            }
        }

        pub trait $TraitName: AsRef<$Kind> {
            $(
                fn $is_a(&self) -> bool {
                    match self.as_ref() {
                        &$Kind::$Variant(_) => true,
                        _ => false
                    }
                }

                fn $get_a<'a>(&'a self) -> Option<&'a $FromKind> {
                    let x: &'a $Kind = self.as_ref();
                    match x {
                        &$Kind::$Variant(ref a) => Some(a.as_ref()),
                        _ => None
                    }
                }
            )*
        }

        impl $TraitName for $Kind { }

        $(
        impl From<$FromKind> for $Kind {
            #[inline(always)]
            fn from(arg: $FromKind) -> Self {
                Self::$Variant(Box::new(arg))
            }
        }
        )*
    };
}

pub mod args;
pub mod assign;
pub mod comparg;
pub mod condition;
pub mod expr;
pub mod func;
pub mod ident;
pub mod invoke;
pub mod kind;
pub mod op;
pub mod statement;
pub mod template;

use crate::internals::parser::ast::args::FunctionArg;
use crate::internals::parser::ast::assign::Assign;
use crate::internals::parser::ast::comparg::{CompositionalFunction, CompositionalFunctionArg};
use crate::internals::parser::ast::condition::Conditional;
use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::func::FunctionDec;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::invoke::Invoke;
use crate::internals::parser::ast::op::Operation;
use crate::internals::parser::ast::statement::{State, Statement};
use crate::internals::parser::ast::template::Template;
use crate::internals::parser::span::{Span, Spanner};

/// Representation is all possible values of
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Representation {
    Statement(Statement),
    Ident(Ident),
    Assign(Assign),
    FunctionArg(FunctionArg),
    Template(Template),
    CompositionalFunctionArg(CompositionalFunctionArg),
    CompositionalFunction(CompositionalFunction),
    Conditional(Conditional),
    Expression(Expression),
    FunctionDec(FunctionDec),
    Invoke(Invoke),
    Operation(Operation),
}

impl AsRef<Representation> for Representation {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Self {
        self
    }
}

impl From<Statement> for Representation {
    fn from(arg: Statement) -> Self {
        Self::Statement(arg)
    }
}

impl From<Ident> for Representation {
    fn from(arg: Ident) -> Self {
        Self::Ident(arg)
    }
}

impl From<Assign> for Representation {
    fn from(arg: Assign) -> Self {
        Self::Assign(arg)
    }
}

impl From<FunctionArg> for Representation {
    fn from(arg: FunctionArg) -> Self {
        Self::FunctionArg(arg)
    }
}

impl From<Template> for Representation {
    fn from(arg: Template) -> Self {
        Self::Template(arg)
    }
}

impl From<CompositionalFunctionArg> for Representation {
    fn from(arg: CompositionalFunctionArg) -> Self {
        Self::CompositionalFunctionArg(arg)
    }
}

impl From<CompositionalFunction> for Representation {
    fn from(arg: CompositionalFunction) -> Self {
        Self::CompositionalFunction(arg)
    }
}

impl From<Conditional> for Representation {
    fn from(arg: Conditional) -> Self {
        Self::Conditional(arg)
    }
}

impl From<Expression> for Representation {
    fn from(arg: Expression) -> Self {
        Self::Expression(arg)
    }
}

impl From<FunctionDec> for Representation {
    fn from(arg: FunctionDec) -> Self {
        Self::FunctionDec(arg)
    }
}

impl From<Invoke> for Representation {
    fn from(arg: Invoke) -> Self {
        Self::Invoke(arg)
    }
}

impl From<Operation> for Representation {
    fn from(arg: Operation) -> Self {
        Self::Operation(arg)
    }
}

impl AsRef<Span> for Representation {
    fn as_ref<'a>(&'a self) -> &'a Span {
        match self {
            &Self::Statement(ref a) => a.as_ref(),
            &Self::Ident(ref a) => a.as_ref(),
            &Self::Assign(ref a) => a.as_ref(),
            &Self::FunctionArg(ref a) => a.as_ref(),
            &Self::Template(ref a) => a.as_ref(),
            &Self::CompositionalFunctionArg(ref a) => a.as_ref(),
            &Self::CompositionalFunction(ref a) => a.as_ref(),
            &Self::Conditional(ref a) => a.as_ref(),
            &Self::Expression(ref a) => a.as_ref(),
            &Self::FunctionDec(ref a) => a.as_ref(),
            &Self::Invoke(ref a) => a.as_ref(),
            &Self::Operation(ref a) => a.as_ref(),
        }
    }
}

impl Spanner for Representation {}

/// Getter and is methods on Representation
pub trait ReprTrait: AsRef<Representation> {
    fn is_statement(&self) -> bool {
        match self.as_ref() {
            &Representation::Statement(_) => true,
            _ => false,
        }
    }
    fn is_ident(&self) -> bool {
        match self.as_ref() {
            &Representation::Ident(_) => true,
            _ => false,
        }
    }
    fn is_assign(&self) -> bool {
        match self.as_ref() {
            &Representation::Assign(_) => true,
            _ => false,
        }
    }
    fn is_function_arg(&self) -> bool {
        match self.as_ref() {
            &Representation::FunctionArg(_) => true,
            _ => false,
        }
    }
    fn is_template(&self) -> bool {
        match self.as_ref() {
            &Representation::Template(_) => true,
            _ => false,
        }
    }
    fn is_compositional_function_arg(&self) -> bool {
        match self.as_ref() {
            &Representation::CompositionalFunctionArg(_) => true,
            _ => false,
        }
    }
    fn is_compositional_function(&self) -> bool {
        match self.as_ref() {
            &Representation::CompositionalFunction(_) => true,
            _ => false,
        }
    }
    fn is_conditional(&self) -> bool {
        match self.as_ref() {
            &Representation::Conditional(_) => true,
            _ => false,
        }
    }
    fn is_expression(&self) -> bool {
        match self.as_ref() {
            &Representation::Expression(_) => true,
            _ => false,
        }
    }
    fn is_function_dec(&self) -> bool {
        match self.as_ref() {
            &Representation::FunctionDec(_) => true,
            _ => false,
        }
    }
    fn is_invoke(&self) -> bool {
        match self.as_ref() {
            &Representation::Invoke(_) => true,
            _ => false,
        }
    }
    fn is_operation(&self) -> bool {
        match self.as_ref() {
            &Representation::Operation(_) => true,
            _ => false,
        }
    }

    fn get_statement<'a>(&'a self) -> Option<&'a Statement> {
        match self.as_ref() {
            &Representation::Statement(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_ident<'a>(&'a self) -> Option<&'a Ident> {
        match self.as_ref() {
            &Representation::Ident(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_assign<'a>(&'a self) -> Option<&'a Assign> {
        match self.as_ref() {
            &Representation::Assign(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_function_arg<'a>(&'a self) -> Option<&'a FunctionArg> {
        match self.as_ref() {
            &Representation::FunctionArg(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_template<'a>(&'a self) -> Option<&'a Template> {
        match self.as_ref() {
            &Representation::Template(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_compositional_function_arg<'a>(&'a self) -> Option<&'a CompositionalFunctionArg> {
        match self.as_ref() {
            &Representation::CompositionalFunctionArg(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_compositional_function<'a>(&'a self) -> Option<&'a CompositionalFunction> {
        match self.as_ref() {
            &Representation::CompositionalFunction(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_conditional<'a>(&'a self) -> Option<&'a Conditional> {
        match self.as_ref() {
            &Representation::Conditional(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_expression<'a>(&'a self) -> Option<&'a Expression> {
        match self.as_ref() {
            &Representation::Expression(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_function_dec<'a>(&'a self) -> Option<&'a FunctionDec> {
        match self.as_ref() {
            &Representation::FunctionDec(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_invoke<'a>(&'a self) -> Option<&'a Invoke> {
        match self.as_ref() {
            &Representation::Invoke(ref arg) => Some(arg),
            _ => None,
        }
    }
    fn get_operation<'a>(&'a self) -> Option<&'a Operation> {
        match self.as_ref() {
            &Representation::Operation(ref arg) => Some(arg),
            _ => None,
        }
    }
}

impl ReprTrait for Representation {}
