//! Example types with all kinds of type parameter
/// type to use as argument for parameterized type
#[derive(Debug, Eq, PartialEq)]
pub enum TAA {
    PAA,
    RAA,
}
#[derive(Debug, Eq, PartialEq)]
pub enum TAB {
    PAB,
    RAB,
    XAB,
}
#[derive(Debug, Eq, PartialEq)]
pub enum TAC {
    PAC,
    RAC,
}
#[derive(Debug, Eq, PartialEq)]
pub enum TBA {
    PBA,
    RBA,
}
#[derive(Debug, Eq, PartialEq)]
pub enum TBB {
    PBB,
    RBB,
    XBB,
}
#[derive(Debug, Eq, PartialEq)]
pub enum TBC {
    PBC,
    RBC,
}

trait MyTrait<A> {
    type B;
    fn method<C>(&self, arg1: A, arg2: Self::B, arg3: C) -> Vec<(&A, &Self::B, &C)>;
}
struct TypeA<T>(T);
impl MyTrait<TAA> for TypeA<TAB> {
    type B = TAB;
    fn method<V>(&self, _arg1: TAA, _arg2: Self::B, _arg3: V) -> Vec<(&TAA, &Self::B, &V)> {
        unimplemented!()
    }
}
struct TypeB<T>(T);
impl MyTrait<TBA> for TypeB<TBB> {
    type B = TBB;
    fn method<V>(&self, _arg1: TBA, _arg2: Self::B, _arg3: V) -> Vec<(&TBA, &Self::B, &V)> {
        unimplemented!()
    }
}
pub fn execute() {
    // let v0 = vec![];
    let va = vec![(&TAA::RAA, &TAB::RAB, &TAC::RAC)];
    let vb = vec![(&TBA::RBA, &TBB::RBB, &TBC::RBC)];

    println!("{}", module_path!());

    println!("  type elision");
    // type inferred by later use
    let a = TypeA(TAB::XAB).method(TAA::PAA, TAB::PAB, TAC::PAC);
    println!("    {}: {} left: {:?}, right: {:?}", line!(), va == a, va, a);
    let b = TypeB(TBB::XBB).method(TBA::PBA, TBB::PBB, TBC::PBC);
    println!("    {}: {} left: {:?}, right: {:?}", line!(), vb == b, vb, b);
}
