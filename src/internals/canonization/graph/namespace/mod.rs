use std::collections::HashMap;
use std::str::FromStr;

/*
 * Type Inference stuff
 *
 */
use crate::internals::canonization::kinds::{
    collection::CollectionTrait,
    primative::{Prim, PrimativeTrait},
    workable::{TypeData, TypeDataTrait},
};

use crate::internals::parser::ast::expr::{Expr as AstExpr, Expression as AstExpression};
use crate::internals::parser::ast::statement::{State as AstState, Statement as AstStatement};
use crate::internals::parser::ast::{GetInternalExpression, InternalExpression};
use crate::internals::parser::span::{Span, Spanner};

mod bind;
pub use self::bind::{Assign, FunctionArg, Variable};
mod func;
pub use self::func::{CompFunction, Func, Function};
mod traits;
pub use self::traits::{GetName, NamespaceError};

/// Namespace contains information about what declarations have occured within
/// a specific scope.
#[derive(Default, Clone)]
pub struct Namespace<'temp, 'input: 'temp> {
    variables: HashMap<&'temp str, Variable<'temp, 'input>>,
    functions: HashMap<&'temp str, Func<'temp, 'input>>,
    internal_namespace: HashMap<&'temp str, Namespace<'temp, 'input>>,
}

impl<'temp, 'input: 'temp> Namespace<'temp, 'input> {
    /// Adds program statements into the namespace
    pub fn add_statement<S, E>(&mut self, statements: S) -> Result<(), E>
    where
        S: IntoIterator<Item = &'temp AstStatement<'input>>,
        E: NamespaceError<'temp, 'input>,
    {
        for statement in statements {
            match &statement.sttm.as_ref() {
                &AstState::Func(ref func) => {
                    self.add_func(func.as_ref())?;
                }
                &AstState::CompFunc(ref comp) => {
                    self.add_func(comp.as_ref())?;
                }
                &AstState::Declaration(ref assign) => {
                    // TODO: Validate expression
                    self.add_variable(assign.as_ref())?;
                }
                &AstState::Termination(_) => {
                    // TODO: Validate expression
                }
            };
        }
        Ok(())
    }

    /*
     * Used in validating the input
     *
     */

    // attempts to infer the type information of an expression
    fn get_type<E>(&self, expr: &'temp AstExpression<'input>) -> Result<TypeData, E>
    where
        E: NamespaceError<'temp, 'input>,
    {
        match expr.get_expr() {
            Option::Some(InternalExpression::Single(expr)) => self.get_type(expr),
            Option::Some(InternalExpression::Op {
                left: ref l,
                right: ref r,
            }) => panic!("TODO"),
            Option::Some(InternalExpression::Conditional {
                cond: ref c,
                true_case: ref t,
                false_case: ref f,
            }) => {
                // validate a conditional
                if !self
                    .get_type(c)?
                    .get_prim()
                    .map(|x| x.is_bool())
                    .unwrap_or_default()
                {
                    // condition part of `if/else` is not a boolean
                    return Err(E::condition_not_bool(expr));
                }

                // ensure both arms return the same type.
                let true_type = self.get_type(t)?;
                let false_type = self.get_type(f)?;

                // check their interior types
                let result = Option::None
                    .into_iter()
                    .chain(
                        true_type
                            .get_prim()
                            .into_iter()
                            .zip(false_type.get_prim())
                            .map(|(t, f)| {
                                ((t.is_int() && f.is_int()) || (t.is_bool() && f.is_bool()))
                            }),
                    )
                    .chain(
                        true_type
                            .get_coll()
                            .into_iter()
                            .zip(false_type.get_coll())
                            .map(|(t, f)| {
                                ((t.contains_int() && f.contains_int())
                                    || (t.contains_bool() && f.contains_bool()))
                            }),
                    )
                    .next()
                    .unwrap_or_default();
                if !result {
                    return Err(E::condition_not_match(expr, t, &true_type, f, &false_type));
                }

                // we validated the types are the same, so just return this
                Ok(true_type)
            }
            Option::None => match expr.kind.as_ref() {
                &AstExpr::Var(ref name) => match self.lookup_type(name.get_span()) {
                    Option::Some(t) => Ok(t.clone()),
                    Option::None => Err(E::var_not_found(expr)),
                },
                &AstExpr::Num(ref num) => {
                    let value = i64::from_str(num.get_span()).unwrap();
                    Ok(TypeData::from(Prim::new_int_constant(value)))
                }
                &AstExpr::Template(_) => panic!("TODO"),
                _ => panic!("should not occur, these types should be transformed"),
            },
        }
    }

    /// looks up a type in a namespace from a valid identifier
    fn lookup_type<'a>(&'a self, arg: &str) -> Option<&'a TypeData> {
        match self.variables.get(arg) {
            Option::Some(var) => return Some(var.as_ref()),
            _ => {}
        };
        match self.functions.get(arg) {
            Option::Some(func) => return Some(func.as_ref()),
            _ => {}
        };
        None
    }

    /*
     * Add variables to the function
     *
     */

    /// attempts to add a variable into the current namespace
    fn add_variable<T, E>(&mut self, var: T) -> Result<(), E>
    where
        Variable<'temp, 'input>: From<T>,
        E: NamespaceError<'temp, 'input>,
    {
        let var = Variable::from(var);

        // create a namespace error
        match self.variables.get(var.get_name()) {
            Option::Some(arg) => {
                let err = E::double_def_var(arg, &var);
                Err(err)
            }
            Option::None => {
                self.variables.insert(var.get_name(), var);
                Ok(())
            }
        }
    }

    /*
     * Add a function to the namespace
     *
     */

    /// attempts to add a function into the current namespace
    fn add_func<T, E>(&mut self, func: T) -> Result<(), E>
    where
        Func<'temp, 'input>: From<T>,
        E: NamespaceError<'temp, 'input>,
    {
        let func = match Func::from(func) {
            Func::Func(f) => {
                // recursively process this function
                self.add_internal(&f)?;
                Func::Func(f)
            }
            Func::Comp(c) => {
                // TODO: check the compositional function
                Func::Comp(c)
            }
        };
        match self.functions.get(func.get_name()) {
            Option::Some(arg) => {
                let err = E::double_def_func(arg, &func);
                Err(err)
            }
            Option::None => {
                self.functions.insert(func.get_name(), func);
                Ok(())
            }
        }
    }

    /*
     * Recursively handle functions
     *
     */

    /// adds an internal namespace
    fn add_internal<E>(&mut self, func: &Function<'temp, 'input>) -> Result<(), E>
    where
        E: NamespaceError<'temp, 'input>,
    {
        // throw error if function already defined
        match self.functions.get(func.get_name()) {
            Option::Some(exists) => {
                return Err(E::double_def_func(exists, func));
            }
            _ => {}
        };

        // build a new namespace
        let mut new_space = Namespace {
            variables: HashMap::default(),
            functions: self.functions.clone(),
            internal_namespace: self.internal_namespace.clone(),
        };

        // add the function's variables to the new namespace
        for arg in func.get_args() {
            new_space.add_variable(arg)?;
        }

        new_space.add_statement(func.get_statements())?;

        // shove the new namespace into our namespace
        match self.internal_namespace.insert(func.get_name(), new_space) {
            Option::Some(_) => {
                panic!("duplicate namespaces should not exist");
            }
            _ => {}
        };

        Ok(())
    }
}
