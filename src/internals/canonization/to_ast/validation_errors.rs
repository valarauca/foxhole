
use crate::internals::parser::{
    span::Spanner,
    ast::{
        template::Template,
        op::Op,
    },
};

// Covers a lot of the errors that can occur while validating the parse tree
pub trait ValidationErrors: Sized {

    fn malformed_int<A,B>(bad_int: &A, expr: &B) -> Self
    where
        A: Spanner,
        B: Spanner;

    /*
     * Template Errors
     *
     */
    fn no_value_for_template(template: &Template) -> Self;
    fn unparsable_template_fallback<S>(template: &Template, value: &S) -> Self
    where
        S: Spanner;
    fn recursive_template_error(template: &Template, interior: Self) -> Self;


    /*
     * Type Errors
     *
     */
    fn no_type_information<S>(arg: &S) -> Self
    where
        S: Spanner;
    fn trinary_op_type_error<L,R>(l: &L, op: &Op, r: &R) -> Self
    where
        L: Spanner,
        R: Spanner;

    /*
     * Namespace collisions
     *
     */
    fn var_conflict<N,O>(new: &N, old: &O) -> Self
    where
        N: Spanner,
        O: Spanner;

    fn func_conflict<N,O>(new: &N, old: &O) -> Self
    where
        N: Spanner,
        O: Spanner;
}
