use crate::internals::parser::ast::op::Op;

use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Boolean {
    maximum: Option<bool>,
    minimum: Option<bool>,
    constant: Option<bool>,
}

/// Propigates type information of a tinary (l op r) expression
/// for boolean operations
pub fn trinary_operations<L,R>(l: &L, op: Op, r: &R) -> Result<Boolean,()>
where
    L: BooleanTrait,
    R: BooleanTrait,
{

    let operation = op_gen(op)?;

    let l_const = l.get_constant();
    let r_const = r.get_constant();

    if l_const.is_some() && r_const.is_some() {
        // simpliest case, both values are constant
        Ok(Boolean::new_constant((operation)(l_const.unwrap(),r_const.unwrap())))
    } else if ( l_const.is_some() && r.has_minimum() && r.has_maximum() ) ||
        ( r_const.is_some() && l.has_minimum() && l.has_maximum()) {
        // one value is constant
        // AND
        // the ther value is bounded
    
        let (con_val, min, max) = if l_const.is_some() {
            (l_const.unwrap(), r.get_minimum().unwrap(), r.get_maximum().unwrap())
        } else {
            (r_const.unwrap(), l.get_minimum().unwrap(), l.get_maximum().unwrap())
        };

        let new_min = (operation)(con_val,min);
        let new_max = (operation)(con_val,max);

        if new_min == new_max {
            Ok(Boolean::new_constant(new_min))
        } else if new_min < new_max {
            Ok(Boolean::new(new_max, new_min, None))
        } else {
            Ok(Boolean::new(new_min, new_max, None))
        }
    } else if op == Op::OR && ( l_const == Option::Some(true) || r_const == Option::Some(true) ) {
        // one value maybe unbounded but we're dealing with an '||' so we can still emit a
        // constant and later trim the tree
        Ok(Boolean::new_constant(true))
    } else {
        Ok(Boolean::new(None,None,None))
    }
}



// returns a function to compute the operation
//
// Boolean results are defined for
// - eq
// - ne
// - and
// - or
// - xor
fn op_gen(op: Op) -> Result<&'static (dyn Fn(bool,bool)->bool+'static),()> {
    match op {
        Op::EQ => {
            fn eq(l: bool, r: bool) -> bool { l == r } 
            Ok(&eq)
        }
        Op::NE => { 
            fn ne(l: bool, r: bool) -> bool { l != r }
            Ok(&ne)
        }
        Op::AND => {
            fn and(l: bool, r: bool) -> bool { l && r }
            Ok(&and)
        }
        Op::OR => {
            fn or(l: bool, r: bool) -> bool { l || r }
            Ok(&or)
        }
        Op::XOR => {
            fn xor(l: bool, r: bool) -> bool { l ^ r }
            Ok(&xor)
        }
        _ => { Err(()) }
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

    pub fn all_vals() -> Self {
        Boolean::new(true, false, None)
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

#[test]
fn test_boolean_propigation() {
    let tests: Vec<(Boolean,Boolean,Op,Result<Boolean,()>)> = vec![
        (Boolean::new_constant(true),Boolean::new_constant(true),Op::AND, Ok(Boolean::new_constant(true))),
        (Boolean::new_constant(false),Boolean::new_constant(true),Op::AND, Ok(Boolean::new_constant(false))),
        (Boolean::new_constant(true),Boolean::new_constant(false),Op::AND, Ok(Boolean::new_constant(false))),
    ];

    for test_case in tests {
        let l = test_case.0;
        let r = test_case.1;
        let op = test_case.2;
        let result = test_case.3;

        let output = trinary_operations(&l,op,&r);
        if output != result {
            panic!("found:{:?} expected:{:?} for ( {:?} {:?} {:?} )", output, result, l, op, r);
        }
    }
}
