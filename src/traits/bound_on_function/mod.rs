//! Example trait with type parameter on function
use super::{Is, TA, TB, TC};
pub trait MyTrait {
    fn arguments<T: 'static>(&self, arg: T);
    fn results<T: 'static>(&self) -> Option<&T>;
    fn both_same<T: 'static>(&self, arg: T) -> Option<&T>;
    fn both_different<T, R: 'static>(&self, arg: T) -> Option<&R>;
}
pub mod polymorphic;
pub mod simple;

pub fn main() {
    self::simple::main();
    self::polymorphic::main();
}
