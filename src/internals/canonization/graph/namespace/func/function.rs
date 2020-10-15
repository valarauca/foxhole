use crate::internals::{
    canonization::{
        graph::namespace::GetName,
        kinds::workable::{TypeData, TypeDataTrait},
    },
    parser::{
        ast::{
            args::FunctionArg as AstFunctionArg, func::FunctionDec as AstFunctionDec,
            statement::Statement as AstStatement,
        },
        span::{Span, Spanner},
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function<'temp, 'input: 'temp> {
    internal: &'temp AstFunctionDec<'input>,
    kind: TypeData,
}

impl<'temp, 'input: 'temp> Function<'temp, 'input> {
    /// returns the arguments of the function
    pub fn get_args(&self) -> &'temp [AstFunctionArg<'input>] {
        self.internal.args.as_slice()
    }

    /// returns statements from
    pub fn get_statements(&self) -> &'temp [AstStatement<'input>] {
        self.internal.body.as_slice()
    }
}

/*
 * Constructor
 *
 */

impl<'temp, 'input: 'temp> From<&'temp AstFunctionDec<'input>> for Function<'temp, 'input> {
    #[inline(always)]
    fn from(arg: &'temp AstFunctionDec<'input>) -> Self {
        Self {
            kind: TypeData::from(arg),
            internal: arg,
        }
    }
}

/*
 * Type System Boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<TypeData> for Function<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        &self.kind
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for Function<'temp, 'input> {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.kind
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for Function<'temp, 'input> {}

/*
 * Way to fetch a name
 *
 */

impl<'temp, 'input: 'temp> GetName<'temp> for Function<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        self.internal.name.get_span()
    }
}

/*
 * Span boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Function<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        self.internal.as_ref()
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for Function<'temp, 'input> {}
