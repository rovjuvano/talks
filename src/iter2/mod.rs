#[macro_use]
pub mod macros;
pub use self::macros::*;

pub mod functions;
pub use self::functions::*;

pub mod helpers;
pub use self::helpers::*;

pub mod iterator;
pub use self::iterator::*;

// #[derive(Debug, Eq, PartialEq)]
#[derive(Clone, Debug, Eq, PartialEq)]
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct X(i32);
impl X {
    pub fn array_owned() -> [X; 3] {
        [X(1), X(2), X(3)]
    }
    pub fn array_borrowed<'a>() -> [&'a X; 3] {
        [&X(1), &X(2), &X(3)]
    }
    pub fn vec_owned() -> Vec<X> {
        vec![X(1), X(2), X(3)]
    }
    pub fn vec_borrowed<'a>() -> Vec<&'a X> {
        vec![&X(1), &X(2), &X(3)]
    }
}
