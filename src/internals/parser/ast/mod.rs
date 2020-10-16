#![allow(dead_code)]

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

/// For representing internal arguments
pub enum InternalExpression<'temp> {
    Single(&'temp Expression),
    Op {
        left: &'temp Expression,
        right: &'temp Expression,
    },
    Conditional {
        cond: &'temp Expression,
        true_case: &'temp Expression,
        false_case: &'temp Expression,
    },
}

/// GetInternalExpression is a useful system for transversing the AST
pub trait GetInternalExpression {
    fn get_expr<'a>(&'a self) -> Option<InternalExpression<'a>>;
}

/// Representation is all possible values of
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Representation<'temp> {
    Statement(&'temp Statement),
    Ident(&'temp Ident),
    Assign(&'temp Assign),
    FunctionArg(&'temp FunctionArg),
    Template(&'temp Template),
    CompositionalFunctionArg(&'temp CompositionalFunctionArg),
    CompositionalFunction(&'temp CompositionalFunction),
    Conditional(&'temp Conditional),
    Expression(&'temp Expression),
    FunctionDec(&'temp FunctionDec),
    Invoke(&'temp Invoke),
    Operation(&'temp Operation),
}

impl<'temp> AsRef<Representation<'temp>> for Representation<'temp> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Self {
        self
    }
}

impl<'temp> From<&'temp Statement> for Representation<'temp> {
    fn from(arg: &'temp Statement) -> Self {
        Self::Statement(arg)
    }
}

impl<'temp> From<&'temp Ident> for Representation<'temp> {
    fn from(arg: &'temp Ident) -> Self {
        Self::Ident(arg)
    }
}

impl<'temp> From<&'temp Assign> for Representation<'temp> {
    fn from(arg: &'temp Assign) -> Self {
        Self::Assign(arg)
    }
}

impl<'temp> From<&'temp FunctionArg> for Representation<'temp> {
    fn from(arg: &'temp FunctionArg) -> Self {
        Self::FunctionArg(arg)
    }
}

impl<'temp> From<&'temp Template> for Representation<'temp> {
    fn from(arg: &'temp Template) -> Self {
        Self::Template(arg)
    }
}

impl<'temp> From<&'temp CompositionalFunctionArg> for Representation<'temp> {
    fn from(arg: &'temp CompositionalFunctionArg) -> Self {
        Self::CompositionalFunctionArg(arg)
    }
}

impl<'temp> From<&'temp CompositionalFunction> for Representation<'temp> {
    fn from(arg: &'temp CompositionalFunction) -> Self {
        Self::CompositionalFunction(arg)
    }
}

impl<'temp> From<&'temp Conditional> for Representation<'temp> {
    fn from(arg: &'temp Conditional) -> Self {
        Self::Conditional(arg)
    }
}

impl<'temp> From<&'temp Expression> for Representation<'temp> {
    fn from(arg: &'temp Expression) -> Self {
        Self::Expression(arg)
    }
}

impl<'temp> From<&'temp FunctionDec> for Representation<'temp> {
    fn from(arg: &'temp FunctionDec) -> Self {
        Self::FunctionDec(arg)
    }
}

impl<'temp> From<&'temp Invoke> for Representation<'temp> {
    fn from(arg: &'temp Invoke) -> Self {
        Self::Invoke(arg)
    }
}

impl<'temp> From<&'temp Operation> for Representation<'temp> {
    fn from(arg: &'temp Operation) -> Self {
        Self::Operation(arg)
    }
}

impl<'temp> AsRef<Span> for Representation<'temp> {
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

impl<'temp> Spanner for Representation<'temp> {}

/// Getter and is methods on Representation
pub trait ReprTrait<'temp>: AsRef<Representation<'temp>> {
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

    fn get_statement(&self) -> Option<&'temp Statement> {
        match self.as_ref() {
            &Representation::Statement(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_ident(&self) -> Option<&'temp Ident> {
        match self.as_ref() {
            &Representation::Ident(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_sssign(&self) -> Option<&'temp Assign> {
        match self.as_ref() {
            &Representation::Assign(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_function_arg(&self) -> Option<&'temp FunctionArg> {
        match self.as_ref() {
            &Representation::FunctionArg(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_template(&self) -> Option<&'temp Template> {
        match self.as_ref() {
            &Representation::Template(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_compositional_function_arg(&self) -> Option<&'temp CompositionalFunctionArg> {
        match self.as_ref() {
            &Representation::CompositionalFunctionArg(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_compositional_function(&self) -> Option<&'temp CompositionalFunction> {
        match self.as_ref() {
            &Representation::CompositionalFunction(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_conditional(&self) -> Option<&'temp Conditional> {
        match self.as_ref() {
            &Representation::Conditional(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_expression(&self) -> Option<&'temp Expression> {
        match self.as_ref() {
            &Representation::Expression(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_function_dec(&self) -> Option<&'temp FunctionDec> {
        match self.as_ref() {
            &Representation::FunctionDec(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_invoke(&self) -> Option<&'temp Invoke> {
        match self.as_ref() {
            &Representation::Invoke(arg) => Some(arg),
            _ => None,
        }
    }
    fn get_operation(&self) -> Option<&'temp Operation> {
        match self.as_ref() {
            &Representation::Operation(arg) => Some(arg),
            _ => None,
        }
    }
}

impl<'temp> ReprTrait<'temp> for Representation<'temp> {}
