
pub mod int;
pub mod bool;

/// Prim is the -simpliest- type. Ultimately most things in
/// foxhole are a boolean or integer. Sometimes collections,
/// or functions which act on them. But all of these higher
/// level concepts require a core primative to exist, I think.
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash)]
pub enum Prim {
    Int,
    Bool
}

