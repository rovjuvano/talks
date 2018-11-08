//! Example trait with type parameter on trait with generic type parameters
use super::{Is, TA, TB, TC};
pub trait MyTrait<T, U> {
    fn arguments(&self, arg: T);
    fn results(&self) -> Option<&T>;
    fn both_same(&self, arg: T) -> Option<&T>;
    fn both_different(&self, arg: T) -> Option<&U>;
}
pub mod generic;
pub mod polymorphic;
pub mod simple;

pub fn main() {
    self::simple::main();
    self::generic::main();
    self::polymorphic::main();
}
