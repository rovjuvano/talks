//! Example types without using traits (inherent methods only)
use super::{Is, TA, TB, TC};
pub mod polymorphic;
pub mod simple;

pub fn main() {
    self::simple::main();
    self::polymorphic::main();
}
