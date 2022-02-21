
use std::collections::HashMap;

use serde::{Serialize};

use crate::internals::parser::{
    ast::{
        ident::Ident,
        template::{Template,TemplateBehavior,TemplateFallback},
    },
    span::{Span,Spanner},
};


/// Handles storing & updating template information
pub struct TemplateDefinations {
    data: HashMap<String,usize>,
}

/// Abstract Error format for when a template error occurs
pub trait TemplateError: Sized {

    fn no_value(template: &Template) -> Self;

    fn unparsable_fallback(template: &Template, value: &Span) -> Self;

    fn recursive(template: &Template, interior: Self) -> Self;
}


impl Default for TemplateDefinations {
    fn default() -> TemplateDefinations {
        Self {
            data: std::env::vars()
                .filter_map(|(key,value)| -> Option<(String,usize)> {
                    usize::from_str_radix(&value, 10)
                        .ok()
                        .map(|v| (key,v))
                })
                .collect::<HashMap<String,usize>>(),
        }
    }
}

impl TemplateDefinations {

    pub fn test_constructor<I,S>(iter: I) -> Self
    where
        String: From<S>,
        I: IntoIterator<Item=(S,usize)>,
    {
        Self {
            data: iter.into_iter().map(|(a,b)| (String::from(a),b)).collect(),
        }
    }

    /// attempts to find value for a template
    pub fn get_value<E>(&mut self, template: &Template) -> Result<usize,E>
    where
        E: TemplateError,
    {
        if let Some(x) = self.lookup_ident(&template.ident) {
            return Ok(x);
        }

        match &template.behavior {
            &Option::None => {
                Err(E::no_value(template))
            },
            &Option::Some(TemplateBehavior::Fallback(TemplateFallback::Num(ref val))) => {
                match usize::from_str_radix(val.get_span(), 10) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(E::unparsable_fallback(template, val)),
                }
            },
            &Option::Some(TemplateBehavior::Assign(TemplateFallback::Num(ref val))) => {
                match usize::from_str_radix(val.get_span(), 10) {
                    Ok(x) => {
                        self.insert_ident(&template.ident, x);
                        Ok(x)
                    }
                    Err(_) => return Err(E::unparsable_fallback(template,val))
                }
            }
            &Option::Some(TemplateBehavior::Fallback(TemplateFallback::Template(ref t))) => {
                // potentially unbounded recursion
                match self.get_value::<E>(t) {
                    Ok(x) => Ok(x),
                    Err(e) => Err(E::recursive(template,e))
                }
            }
            &Option::Some(TemplateBehavior::Assign(TemplateFallback::Template(ref t))) => {
                // potentially unbounded recursion
                match self.get_value::<E>(t) {
                    Ok(x) => {
                        self.insert_ident(&template.ident, x);
                        Ok(x)
                    }
                    Err(e) => Err(E::recursive(template,e))
                }
            }
        }
    }

    fn insert_ident(&mut self, ident: &Ident, value: usize) {
        self.data.insert(ident.get_span().to_string(), value);
    }
    
    fn lookup_ident(&self, ident: &Ident) -> Option<usize> {
        self.data.get(ident.get_span()).map(|x| x.clone())
    }
}
