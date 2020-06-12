

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum Prim {
    Int,
    Bool,
}

/// Kind is a very high level representation of type information.
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum Kind {
    Prim(Prim),
    Coll(Prim),
}


pub const C_I: Kind = Kind::Coll(Prim::Int);
pub const C_B: Kind = Kind::Coll(Prim::Bool);
pub const I: Kind = Kind::Prim(Prim::Int);
pub const B: Kind = Kind::Prim(Prim::Bool);

/// TypeInfoTrait is a high level trait used to present
/// information about the type itself.
pub trait TypeInfoTrait {

    /// returns the type information
    fn get_type(&self) -> Kind;

    /*
     * These methods are derived from `get_type`
     *
     */

    fn is_collection(&self) -> bool {
        match self.get_type() {
            Kind::Coll(_) => true,
            _ => false
        }
    }

    fn is_primative(&self) -> bool {
        match self.get_type() {
            Kind::Prim(_) => true,
            _ => false
        }
    }

    fn is_collection_of_integer(&self) -> bool {
        match self.get_type() {
            Kind::Coll(Prim::Int) => true,
            _ => false,
        }
    }

    fn is_collection_of_boolean(&self) -> bool {
        match self.get_type() {
            Kind::Coll(Prim::Bool) => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self.get_type() {
            Kind::Prim(Prim::Int) => true,
            _ => false,
        }
    }

    fn is_boolean(&self) -> bool {
        match self.get_type() {
            Kind::Prim(Prim::Bool) => true,
            _ => false,
        }
    }

    /// stat info may not necessarily be avaliable
    /// depending on the current phase of compilation
    fn get_stats(&self) -> Option<TypeInfo> {
        None
    }

    fn collection_has_fixed_size(&self) -> Option<bool> {
        match self.get_stats() {
            Option::None => None,
            Option::Some(TypeInfo::Coll(ref c_info)) => {
                c_info.max_size.clone().into_iter().zip(c_info.min_size.clone
                match (&c_info.max_size,&c_info.min_size) {
                    
                }
            }
        }
    }
}

impl TypeInfoTrait for Kind {

    #[inline]
    fn get_type(&self) -> Kind {
        self.clone()
    }
}

/// TypeInfo contains more advanced runtime data.
///
/// Specifically about sizes & range limits of the respective types.
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum TypeInfo {
    Int(IntRange),
    Bool(BoolRange),
    Coll(CollectionInfo),
}

impl TypeInfoTrait for TypeInfo {

    #[inline]
    fn get_type(&self) -> Kind {
        match self {
            &TypeInfo::Int(_) => I,
            &TypeInfo::Bool(_) => B,
            &TypeInfo::Coll(ref x) => match &x.data {
                &CollectionData::Int(_) => C_I,
                &CollectionData::Bool(_) => C_B,
            }
        }
    }

    #[inline]
    fn get_stats(&self) -> Option<TypeInfo> {
        Some(self.clone())
    }
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum CollectionData {
    Int(IntRange),
    Bool(BoolRange),
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct CollectionInfo {
    pub max_size: Option<i64>,
    pub min_size: Option<i64>,
    pub data: CollectionData,
}
impl CollectionInfo {

    fn has_fixed_size(&self) -> bool {
        self.max_size.clone().into_iter().zip(self.min_size.clone()).map(|(max,min) max.eq(&min)).next().unwrap_or_else(|| false)
    }
}

#[derive(Default,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct BoolRange {
    pub always_true: bool,
    pub always_false: bool,
}

#[derive(Default,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct IntRange {
    pub max: i64,
    pub min: i64,
}
