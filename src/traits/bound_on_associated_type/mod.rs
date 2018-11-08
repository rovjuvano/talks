//! Examples traits with type parameter as associate type
use super::{Is, TA, TB, TC};
pub trait MyTrait {
    type T;
    type U;
    fn arguments(&self, arg: Self::T);
    fn results(&self) -> Option<&Self::T>;
    fn both_same(&self, arg: Self::T) -> Option<&Self::T>;
    fn both_different<'a>(&self, arg: Self::T) -> Option<&'a Self::U>;
}
pub mod generic;
pub mod polymorphic;
pub mod simple;

pub fn main() {
    self::simple::main();
    self::generic::main();
    self::polymorphic::main();
}
