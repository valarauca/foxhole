
/// Integer is the basic representation of an integer value.
///
/// It holds information concerning the maximum, or minimum,
/// and variability of the value.
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct Integer {
    maximum: Option<i64>,
    minimum: Option<i64>,
    constant_value: Option<i64>,
}
