use crate::internals::parser::ast::op::Op;

use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Boolean {
    maximum: Option<bool>,
    minimum: Option<bool>,
    constant: Option<bool>,
}


pub fn trinary_operations<L,R>(l: &L, op: Op, r: &R) -> Result<Boolean,()>
where
    L: BooleanTrait,
    R: BooleanTrait,
{
    match op {
        Op::ADD | Op::SUB | Op::MUL | Op::DIV => {
            Err(())
        }
        Op::EQ => {
            if l.is_constant() && r.is_constant() {

            } else {
            }
        }
    }
}

impl AsRef<Boolean> for Boolean { 
    fn as_ref<'a>(&'a self) -> &'a Self { self }
} 
impl AsMut<Boolean> for Boolean {
    fn as_mut<'a>(&'a mut self) -> &'a mut Self { self } 
}

impl Boolean {
    pub fn new_constant(value: bool) -> Self {
       Self { maximum: None, minimum: None, constant: Some(value) }
    }

    pub fn new<Max, Min, Const>(max: Max, min: Min, con: Const) -> Self
    where
        Max: Into<Option<bool>>,
        Min: Into<Option<bool>>,
        Const: Into<Option<bool>>,
    {
        let mut value = Boolean::default();
        value.set_maximum(max);
        value.set_minimum(min);
        value.set_constant(con);

        value
    }
}

pub trait BooleanTrait: AsRef<Boolean> {

    fn get_maximum(&self) -> Option<bool> {
        self.as_ref().maximum.clone()
    }
    fn has_maximum(&self) -> bool {
        self.get_maximum().is_some()
    }

    fn get_minimum(&self) -> Option<bool> {
        self.as_ref().minimum.clone()
    }
    fn has_minimum(&self) -> bool {
        self.get_maximum().is_some()
    }

    fn get_constant(&self) -> Option<bool> {
        self.as_ref().constant.clone()
    }
    fn has_constant(&self) -> bool {
        self.get_maximum().is_some()
    }
    fn is_constant(&self) -> bool {
        self.has_constant()
            &&
        self.has_minimum()
            &&
        self.has_maximum()
            &&
        self.get_minimum() == self.get_maximum()
            &&
        self.get_constant() == self.get_maximum()
    }
}

pub trait BooleanMutTrait: AsMut<Boolean> + BooleanTrait {

    fn set_minimum<Min>(&mut self, min: Min)
    where
        Min: Into<Option<bool>>,
    {
        match min.into() {
            Option::None => {
                // we are just invalidating the minimum value
                //
                // minimum is now unbounded.
                // constant is now invalidated.
                self.as_mut().minimum = None;
                self.as_mut().constant = None;
            }
            Option::Some(new_min) => {
                // we have a new minimum value, we need to
                // see how it is related to maximum
                match self.get_maximum() {
                    Option::Some(max) if max < new_min => {
                        // maximum is now invalidated
                        // meaning the maximum range is now "endless"
                        self.as_mut().maximum = None;
                        self.as_mut().constant = None;
                    }
                    Option::Some(max) if max == new_min => {
                        // maximum is now equal to minimum
                        // meaning we infer the value is constant
                        self.as_mut().constant = Some(new_min);
                    }
                    Option::Some(max) if new_min < max => {
                        // the ordering is correct, but they aren't equal
                        self.as_mut().constant = None;
                    }
                    _ => {
                        // other cases are not of interest
                    }
                };
                self.as_mut().minimum = Some(new_min);
            }
        };
    }

    fn set_maximum<Max>(&mut self, max: Max)
    where
        Max: Into<Option<bool>>,
    {
        match max.into() {
            Option::None => {
                self.as_mut().maximum = None;
                self.as_mut().constant = None;
            }
            Option::Some(new_max) => {
                // we have a new maximum value, we need to
                // see how it is related to minimum
                match self.get_minimum() {
                    Option::Some(min) if min > new_max => {
                        // minimum is now invalidated
                        // meaning the minimum range is now "endless"
                        self.as_mut().minimum = None;
                        self.as_mut().constant = None;
                    }
                    Option::Some(min) if min == new_max => {
                        // maximum is now equal to minimum
                        // meaning we infer the value is constant
                        self.as_mut().constant = Some(new_max);
                    }
                    Option::Some(min) if min < new_max => {
                        // the othering is correct, but they aren't
                        // equal so the constant is invalidated
                        self.as_mut().constant = None;
                    }
                    _ => {
                        // other cases are not of interest
                    }
                };
                self.as_mut().maximum = Some(new_max);
            }
        };
    }

    fn set_constant<Const>(&mut self, con: Const)
    where
        Const: Into<Option<bool>>,
    {
        match con.into() {
            Option::Some(value) => {
                self.as_mut().constant = Some(value);
                self.as_mut().minimum = Some(value);
                self.as_mut().maximum = Some(value);
            }
            Option::None => {
                self.as_mut().constant = None;
            }
        }
    }
}

impl BooleanTrait for Boolean { }
impl BooleanMutTrait for Boolean { }
