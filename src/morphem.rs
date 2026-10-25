use std::ops::{Deref, DerefMut};

#[derive(Clone,PartialEq,Eq)]
pub struct Morphem(String);

impl DerefMut for Morphem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Morphem {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Hash,Clone,Copy,PartialEq,Eq)]
pub enum MorphemType {
    Prefix,
    Core,
    Suffix,
    Ending,
    Postfix,
}
