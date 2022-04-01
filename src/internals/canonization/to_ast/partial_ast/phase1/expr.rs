
use crate::internals::{
    parser::{
        ast::{
            expr::{Expression,Expr},
            template::Template,
        },
        span::{Span,Spanner},
    },
    canonization::{
        to_ast::template_stuff::{TemplateDefinations,TemplateError},
        kinds::{
            workable::{TypeDataTrait,TypeData},
        },
    }
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct P1Expression {
    pub span: Box<Span>,
    pub kind: TypeData,
}

/*
 * Inheriat shit
 *
 */
impl AsRef<Span> for P1Expression { 
    fn as_ref(&self) -> &Span { &self.span }
}
impl Spanner for P1Expression { }


/*
fn to_p1_expr(e: &Expression, t_def: &mut TemplateDefinations) -> Result<P1Expression,()>
{
    match &e.expr {
        &Expr::Template(ref template) => {
            match t_def.get_value::
        }
    }
}
*/
