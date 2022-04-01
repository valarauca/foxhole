use serde::{Deserialize, Serialize};

use crate::internals::{
    parser::{
        ast::{
            expr::{Expression,Expr},
            template::Template,
        },
        span::{Span,Spanner},
    },
    canonization::{
        to_ast::{
            template_stuff::TemplateDefinations,
            validation_errors::ValidationErrors,
        },
        kinds::{
            workable::{TypeDataTrait,TypeData},
            primative::Prim,
        },
    }
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct P1Expression {
    pub span: Box<Span>,
    pub kind: Box<TypeData>,
    pub expr: P1Expr,
}

impl P1Expression {

    // abstract away some boilerplate when making this type
    #[inline(always)]
    fn new<S,T>(span: &S, kind: T, expr: P1Expr) -> Self
    where
        S: Spanner,
        TypeData: From<T>,
    {
        Self {
            span: Box::new(span.get_clone()),
            kind: Box::new(TypeData::from(kind)),
            expr,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum P1Expr {
    Value(i64),
    Parens(Box<P1Expression>),
}

/*
 * Inheriat shit
 *
 */
impl AsRef<Span> for P1Expression { 
    fn as_ref(&self) -> &Span { &self.span }
}
impl Spanner for P1Expression { }

impl AsRef<TypeData> for P1Expression {
    fn as_ref(&self) -> &TypeData { &self.kind }
}
impl AsMut<TypeData> for P1Expression {
    fn as_mut(&mut self) -> &mut TypeData { &mut self.kind }
}
impl TypeDataTrait for P1Expression { }


fn to_p1_expr<E>(
    e: &Expression,
    t_def: &mut TemplateDefinations) -> Result<P1Expression,E>
where
    E: ValidationErrors,
{
    match e.kind.as_ref() {
        &Expr::Template(ref template) => {
            let x = t_def.get_value::<E>(template)?;
            Ok(P1Expression::new( e, Prim::new_int_constant(x as i64), P1Expr::Value(x as i64)))
        },
        &Expr::Num(ref span) => {
            let x = match i64::from_str_radix(span.get_span(), 10) {
                Ok(x) => x,
                Err(_) => return Err(E::malformed_int(span.as_ref(), e))
            };
            Ok(P1Expression::new(e, Prim::new_int_constant(x as i64), P1Expr::Value(x as i64)))
        },
        &Expr::Parens(ref expression) => {
            let inner = to_p1_expr(expression, t_def)?;
            let data: TypeData = inner.get_copy();
            Ok(P1Expression::new(e, data, P1Expr::Parens(Box::new(inner))))
        }
        _ => panic!()
    }
}
