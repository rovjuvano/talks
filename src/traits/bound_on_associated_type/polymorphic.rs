//! failed example trait with type parameter as associate type with multiple implementations
// use super::MyTrait;
// use super::{TA, TB, TC};
// pub struct TypeAandB;
// impl MyTrait for TypeAandB {
//   type T=TA;
//   type U=TC;
//   fn arguments(&self, arg: Self::T) { unimplemented!() }
//   fn results(&self) -> Option<&Self::T> { unimplemented!() }
//   fn both_same(&self, arg: Self::T) -> Option<&Self::T> { unimplemented!() }
//   fn both_different(&self, arg: Self::T) -> Option<&Self::U> { unimplemented!() }
// }
// impl MyTrait for TypeAandB {
//   type T=TB;
//   type U=TC;
//   fn arguments(&self, arg: Self::T) { unimplemented!() }
//   fn results(&self) -> Option<&Self::T> { unimplemented!() }
//   fn both_same(&self, arg: Self::T) -> Option<&Self::T> { unimplemented!() }
//   fn both_different(&self, arg: Self::T) -> Option<&Self::U> { unimplemented!() }
// }
pub fn main() {
    assert!(!false, "ERROR: cannot implement trait multiple times on same type");
    assert!(!false, "ERROR: E0119: conflicting implementations of trait");
}
