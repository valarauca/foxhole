use std::ops::RangeInclusive;
use crate::internals::{
    parser::{ ast::op::Op, span::Spanner},
    canonization::kinds::primative::boolean::{Boolean,BooleanTrait},
};

use serde::{Deserialize, Serialize};


/// Integer is the basic representation of an integer value.
///
/// It holds information concerning the maximum, or minimum,
/// and variability of the value.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Integer {
    maximum: Option<i64>,
    minimum: Option<i64>,
    constant: Option<i64>,
}

pub fn trinary_iib_op<L,R>(l: &L, op: Op, r: &R) -> Result<Boolean,()>
where
    L: IntegerTrait,
    R: IntegerTrait,
{
    let operation = bool_op_gen(op)?;

    // general constant case
    // (works for everything)
    let con = l.get_constant().into_iter().zip(r.get_constant()).map(|(l,r)| operation(l,r)).next();
    if let Option::Some(val) = con {
        return Ok(Boolean::new_constant(val));
    }

    let l_range = l.get_range();
    let r_range = r.get_range();
    match op {
        Op::EQ => {
            if l.is_constant() && !r_range.contains(&l.get_constant().unwrap()) ||
                r.is_constant() && !l_range.contains(&r.get_constant().unwrap()) {
                Ok(Boolean::new_constant(false))
            } else {
                Ok(Boolean::all_vals())
            }
        },
        Op::NE => {
            if l.is_constant() && !r_range.contains(&l.get_constant().unwrap()) ||
                r.is_constant() && !l_range.contains(&r.get_constant().unwrap()) {
                Ok(Boolean::new_constant(true))
            } else {
                Ok(Boolean::all_vals())
            }
        }
        Op::GT | Op::GE | Op::LT | Op::LE => {
            let a = (operation)(*l_range.start(),*r_range.start());
            let b = (operation)(*l_range.start(),*r_range.end());
            let c = (operation)(*l_range.end(), *r_range.start());
            let d = (operation)(*l_range.end(), *r_range.end());
            if a == b && b == c && d == c {
                Ok(Boolean::new_constant(a))
            } else {
                Ok(Boolean::all_vals())
            }
        }
        _ => Err(())
    }
}

pub fn trinary_iii_op<L,R>(l: &L, op: Op, r: &R) -> Result<Integer,()>
where
    L: IntegerTrait,
    R: IntegerTrait,
{

    let op = int_op_gen(op)?;

    let con = l.get_constant().into_iter().zip(r.get_constant()).map(|(l,r)| op(l,r)).next();
    if let Option::Some(x) = con.clone() {
        return Ok(Integer::new_constant(x));
    }

    let max = l.get_maximum().into_iter().zip(r.get_maximum()).map(|(l,r)| op(l,r)).next();
    let min = l.get_minimum().into_iter().zip(r.get_minimum()).map(|(l,r)| op(l,r)).next();
    // check if order changed
    let (max,min) = if max > min { (max,min) } else { (min,max) };
    Ok(Integer::new(max, min, None))
}

fn int_op_gen(op: Op) -> Result<&'static (dyn Fn(i64,i64)->i64+'static),()> {
    match op {
        Op::ADD => {
            fn add(l: i64, r: i64) -> i64 { l + r }
            Ok(&add)
        }
        Op::SUB => {
            fn sub(l: i64, r: i64) -> i64 { l - r }
            Ok(&sub)
        }
        Op::MUL => {
            fn mul(l: i64, r: i64) -> i64 { l * r }
            Ok(&mul)
        }
        Op::DIV => {
            fn div(l: i64, r: i64) -> i64 { l / r }
            Ok(&div)
        }
        _ => {
            Err(())
        }
    }
}

fn bool_op_gen(op: Op) -> Result<&'static (dyn Fn(i64,i64)->bool+'static),()> {
    match op {
        Op::EQ => {
            fn eq(l: i64, r: i64) -> bool { l == r }
            Ok(&eq)
        }
        Op::NE => {
            fn ne(l: i64, r: i64) -> bool { l != r }
            Ok(&ne)
        }
        Op::GT => {
            fn gt(l: i64, r: i64) -> bool { l > r }
            Ok(&gt)
        }
        Op::GE => {
            fn ge(l: i64, r: i64) -> bool { l >= r }
            Ok(&ge)
        }
        Op::LT => {
            fn lt(l: i64, r: i64) -> bool { l < r }
            Ok(&lt)
        }
        Op::LE => {
            fn le(l: i64, r: i64) -> bool { l <= r }
            Ok(&le)
        }
        _ => {
            Err(())
        }
    }
}

impl AsRef<Integer> for Integer {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Integer {
        self
    }
}

impl AsMut<Integer> for Integer {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Integer {
        self
    }
}

impl IntegerTrait for Integer {}
impl IntegerMutTrait for Integer {}

impl Integer {

    /// builds a constant from a span, wrapper function
    /// should handle the error
    pub fn constant_from_span<S>(arg: &S) -> Option<Self>
    where
        S: Spanner,
    {
        i64::from_str_radix(arg.get_span(), 10)
            .ok()
            .map(Self::new_constant)
    }

    /// Create a new instant of `Integer` from a constant value, for a constant
    /// value.
    pub fn new_constant(value: i64) -> Self {
        Self {
            maximum: Some(value),
            minimum: Some(value),
            constant: Some(value),
        }
    }

    /// Create new instant of `Integer` with -possibly- known bounds.
    pub fn new<Max, Min, Const>(max: Max, min: Min, con: Const) -> Self
    where
        Max: Into<Option<i64>>,
        Min: Into<Option<i64>>,
        Const: Into<Option<i64>>,
    {
        let mut value = Self {
            maximum: None,
            minimum: None,
            constant: None,
        };
        value.set_maximum(max);
        value.set_minimum(min);
        value.set_constant(con);
        value
    }
}

/// IntegerTrait defines operations upon the Integer TypeClass.
pub trait IntegerTrait: AsRef<Integer> {
    /// returns the maximum value this integer may contain.
    fn get_maximum(&self) -> Option<i64> {
        self.as_ref().maximum.clone()
    }

    /// returns if a known maximum value exists
    fn has_maximum(&self) -> bool {
        self.as_ref().get_maximum().is_some()
    }

    /// returns the minimum value this integer may contain
    fn get_minimum(&self) -> Option<i64> {
        self.as_ref().minimum.clone()
    }

    /// returns if a minimum value is known
    fn has_minimum(&self) -> bool {
        self.as_ref().get_minimum().is_some()
    }

    /// returns the constant value, if it exists
    fn get_constant(&self) -> Option<i64> {
        self.as_ref().constant.clone()
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

    /// returns if a constant value, is known.
    fn has_constant(&self) -> bool {
        self.as_ref().get_constant().is_some()
    }

    /// returns `(max,min)`, if they are known.
    fn get_bounds(&self) -> Option<(i64, i64)> {
        self.as_ref()
            .get_maximum()
            .into_iter()
            .zip(self.as_ref().get_minimum())
            .next()
    }

    /// returns if the integer has known bounds
    fn has_bounds(&self) -> bool {
        self.as_ref().get_bounds().is_some()
    }

    /// returns the total range
    fn get_range(&self) -> RangeInclusive<i64> {
        let max = self.get_maximum().unwrap_or_else(|| i64::MAX);
        let min = self.get_minimum().unwrap_or_else(|| i64::MIN);
        debug_assert!(max > min);
        RangeInclusive::new(min, max)
    }
}

/// IntegerMutTrait handles mutable functions of the IntegerType
pub trait IntegerMutTrait: AsMut<Integer> + IntegerTrait {
    /// for the integer to have a minimum value
    fn set_minimum<Min>(&mut self, min: Min)
    where
        Min: Into<Option<i64>>,
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

    /// set for the integer to have a maximum value.
    ///
    /// If this value is the same as the current `minimum`
    /// it will update the constant value.
    fn set_maximum<Max>(&mut self, max: Max)
    where
        Max: Into<Option<i64>>,
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

    /// set the constant value.
    ///
    /// If this value is `Some`, then it will also
    /// update the maximum & minimum
    fn set_constant<Const>(&mut self, con: Const)
    where
        Const: Into<Option<i64>>,
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


#[test]
fn trivial_integer_properities() {
    let mut int = Integer::new_constant(5);

    /*
     * Assert the Properites of the constant
     *
     */
    assert!(int.has_constant());
    assert!(int.is_constant());
    assert_eq!(Option::Some(5i64), int.get_constant());
    assert!(int.has_maximum());
    assert_eq!(Option::Some(5i64), int.get_maximum());
    assert!(int.has_minimum());
    assert_eq!(Option::Some(5i64), int.get_minimum());
    assert!(int.has_bounds());
    assert_eq!(Option::Some((5i64, 5i64)), int.get_bounds());

    /*
     * Modify the minimum value
     *
     * Ensure things work properly
     *
     */

    int.set_minimum(4);
    assert!(!int.has_constant());
    assert!(!int.is_constant());
    assert_eq!(Option::None, int.get_constant());
    assert!(int.has_maximum());
    assert_eq!(Option::Some(5i64), int.get_maximum());
    assert!(int.has_minimum());
    assert_eq!(Option::Some(4i64), int.get_minimum());
    assert!(int.has_bounds());
    assert_eq!(Option::Some((5i64, 4i64)), int.get_bounds());

    /*
     * Modify the maximum value
     *
     * Ensure things work properly
     *
     */

    int.set_maximum(6);
    assert!(!int.has_constant());
    assert_eq!(Option::None, int.get_constant());
    assert!(int.has_maximum());
    assert_eq!(Option::Some(6i64), int.get_maximum());
    assert!(int.has_minimum());
    assert_eq!(Option::Some(4i64), int.get_minimum());
    assert!(int.has_bounds());
    assert_eq!(Option::Some((6i64, 4i64)), int.get_bounds());
}

#[test]
fn non_trivial_integer_properites() {
    /*
     * Create a simple constant
     *
     */
    let mut int = Integer::new_constant(6);
    assert!(int.has_constant());
    assert_eq!(Option::Some(6i64), int.get_constant());
    assert!(int.has_maximum());
    assert_eq!(Option::Some(6i64), int.get_maximum());
    assert!(int.has_minimum());
    assert_eq!(Option::Some(6i64), int.get_minimum());
    assert!(int.has_bounds());
    assert_eq!(Option::Some((6i64, 6i64)), int.get_bounds());

    /*
     * Set a minimum very high
     *
     */

    int.set_minimum(8i64);

    // integer isn't a constant
    assert!(!int.has_constant());
    // no longer has a maximum
    assert!(!int.has_maximum());
    // minimum still exists
    assert!(int.has_minimum());
    assert_eq!(Option::Some(8i64), int.get_minimum());
}
