//! failed example trait with type parameter on function with multiple implementations
// use super::MyTrait;
// pub struct TypeAandB;
// impl MyTrait for TypeAandB {
//     fn arguments<T: 'static>(&self, arg: T) { unimplemented!() }
//     fn results<T: 'static>(&self) -> Option<&T> { unimplemented!() }
//     fn both_same<T: 'static>(&self, arg: T) -> Option<&T> { unimplemented!() }
//     fn both_different<T, U: 'static>(&self, arg: T) -> Option<&U> { { unimplemented!() }
// }
// impl MyTrait for TypeAandB {
//     fn arguments<T: 'static>(&self, arg: T) { unimplemented!() }
//     fn results<T: 'static>(&self) -> Option<&T> { unimplemented!() }
//     fn both_same<T: 'static>(&self, arg: T) -> Option<&T> { unimplemented!() }
//     fn both_different<T, U: 'static>(&self, arg: T) -> Option<&U> { { unimplemented!() }
// }
pub fn main() {
    assert!(!false, "ERROR: cannot implement trait multiple times on same type");
    assert!(!false, "ERROR: E0119: conflicting implementations of trait");
}
