//! Example types without using traits (inherent methods only) - multiple implementations
// pub struct TypeAandB;
// impl TypeAandB {
//     pub fn arguments(&self, _arg: TA) {}
//     pub fn arguments(&self, _arg: TB) {}
//     pub fn results(&self) -> Option<&TB> { unimplemented!() }
//     pub fn results(&self) -> Option<&TB> { unimplemented!() }
//     pub fn both_same(&self, _arg: TA) -> Option<&TA> { unimplemented!() }
//     pub fn both_same(&self, _arg: TB) -> Option<&TB> { unimplemented!() }
//     pub fn both_different(&self, _arg: TC) -> Option<&TA> { unimplemented!() }
//     pub fn both_different(&self, _arg: TC) -> Option<&TB> { unimplemented!() }
// }
pub fn main() {
    assert!(!false, "ERROR: cannot implement method with same name and different arguments or return types");
    assert!(!false, "ERROR: E0201: duplicate definitions");
}
