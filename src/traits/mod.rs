//! Exploring the 3 kinds of parameterized types within traits
//!
//! ```
//! trait MyTrait<A> {
//!     type B;
//!     fn method<C>(&self, arg1: A, arg2: Self::B, arg3: C) -> (A, Self::B, C);
//! }
//! struct MyType<T> { ... }
//! impl<T, U> MyTrait<T> for MyType<U> {
//!     type B = U;
//!     fn method<V>(&self, _arg1: T, _arg2: Self::B, _arg3: V) -> (T, U, V) {
//!         unimplemented!()
//!     }
//! }
//! ```
//!
//! # Summary
//!
//! | use            | no traits  | A: trait      | B: assoc. type | C: function    |
//! |----------------|------------|---------------|----------------|----------------|
//! | type at let    | inferred   | placeholder   | inferred       | placeholder    |
//! | turbofish      | no         | no            | no             | yes            |
//! | generic impl   | no         | yes           | yes - E0207    | yes            |
//! | multiple impls | 1/per type | many/type     | 1/per type     | 1 generic/type |
//!
//! # Next
//!
//! Trait bounds

pub trait Is {
    type T;
    fn is(x: Option<&Self::T>) -> bool;
}
/// type to use as argument for parameterized type
#[derive(Debug, Eq, PartialEq)]
pub struct TA(usize);
impl Is for TA {
    type T = TA;
    fn is(x: Option<&TA>) -> bool {
        x == Some(&TA(1))
    }
}
/// type to use as argument for parameterized type
#[derive(Debug, Eq, PartialEq)]
pub struct TB(usize);
impl Is for TB {
    type T = TB;
    fn is(x: Option<&TB>) -> bool {
        x == Some(&TB(2))
    }
}
/// type to use as argument for parameterized type
#[derive(Debug, Eq, PartialEq)]
pub struct TC(usize);
impl Is for TC {
    type T = TC;
    fn is(x: Option<&TC>) -> bool {
        x == Some(&TC(3))
    }
}
impl Is for i32 {
    type T = i32;
    fn is(x: Option<&i32>) -> bool {
        x == Some(&32i32)
    }
}

pub mod bound_on_associated_type;
pub mod bound_on_function;
pub mod bound_on_trait;
pub mod no_traits;

pub fn main() {
    self::no_traits::main();
    self::bound_on_function::main();
    self::bound_on_trait::main();
    self::bound_on_associated_type::main();
}
