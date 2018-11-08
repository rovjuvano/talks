//! Example trait with type parameter as associate type with basic implementation
use super::MyTrait;
use super::{Is, TA, TB, TC};
pub struct TypeA;
impl MyTrait for TypeA {
    type T = TA;
    type U = TC;
    fn arguments(&self, _arg: Self::T) {}
    fn results(&self) -> Option<&Self::T> {
        Some(&TA(1))
    }
    fn both_same(&self, _arg: Self::T) -> Option<&Self::T> {
        Some(&TA(1))
    }
    fn both_different<'a>(&self, _arg: Self::T) -> Option<&'a Self::U> {
        Some(&TC(3))
    }
}
pub struct TypeB;
impl MyTrait for TypeB {
    type T = TB;
    type U = TC;
    fn arguments(&self, _arg: Self::T) {}
    fn results(&self) -> Option<&Self::T> {
        Some(&TB(2))
    }
    fn both_same(&self, _arg: Self::T) -> Option<&Self::T> {
        Some(&TB(2))
    }
    fn both_different<'a>(&self, _arg: Self::T) -> Option<&'a Self::U> {
        Some(&TC(3))
    }
}
pub fn main() {
    type_elision();
    explicitly_typed();
    type_cast();
    turbofish();
}
fn type_elision() {
    arguments_type_elision();
    results_type_elision();
    both_same_type_elision();
    both_different_type_elision();
}
fn arguments_type_elision() {
    TypeA.arguments(TA(0));
    assert!(true, "INFO: types specified in trait impl");

    TypeB.arguments(TB(0));
    assert!(true, "INFO: types specified in trait impl");
}
fn results_type_elision() {
    let a = TypeA.results();
    assert!(TA::is(a), "INFO: types specified in trait impl");

    let b = TypeB.results();
    assert!(TB::is(b), "INFO: types specified in trait impl");
}
fn both_same_type_elision() {
    let a = TypeA.both_same(TA(0));
    assert!(TA::is(a), "INFO: types specified in trait impl");

    let b = TypeB.both_same(TB(0));
    assert!(TB::is(b), "INFO: types specified in trait impl");
}
fn both_different_type_elision() {
    let a = TypeA.both_different(TA(0));
    assert!(TC::is(a), "INFO: types specified in trait impl");

    let b = TypeB.both_different(TB(0));
    assert!(TC::is(b), "INFO: types specified in trait impl");
}
fn explicitly_typed() {
    arguments_explicitly_typed();
    results_explicitly_typed();
    both_same_explicitly_typed();
    both_different_explicitly_typed();
}
fn arguments_explicitly_typed() {
    TypeA.arguments(TA(0));
    assert!(true, "INFO: no need for explicit type: types specified in trait impl");

    TypeB.arguments(TB(0));
    assert!(true, "INFO: no need for explicit type: types specified in trait impl");

    // TypeA.arguments(32i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn results_explicitly_typed() {
    let a: Option<&TA> = TypeA.results();
    assert!(TA::is(a), "INFO: no need for explicit type: types specified in trait impl");

    let b: Option<&TB> = TypeB.results();
    assert!(TB::is(b), "INFO: no need for explicit type: types specified in trait impl");

    // let x: Option<&i32> = TypeA.results();
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn both_same_explicitly_typed() {
    let a: Option<&TA> = TypeA.both_same(TA(0));
    assert!(TA::is(a), "INFO: no need for explicit type: types specified in trait impl");

    let b: Option<&TB> = TypeB.both_same(TB(0));
    assert!(TB::is(b), "INFO: no need for explicit type: types specified in trait impl");

    // let x: Option<&i32> = TypeA.both_same(32i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn both_different_explicitly_typed() {
    let a: Option<&TC> = TypeA.both_different(TA(0));
    assert!(TC::is(a), "INFO: no need for explicit type: types specified in trait impl");

    let b: Option<&TC> = TypeB.both_different(TB(0));
    assert!(TC::is(b), "INFO: no need for explicit type: types specified in trait impl");

    // let x: Option<&i32> = TypeA.both_different(64u64);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn type_cast() {
    arguments_type_cast();
    results_type_cast();
    both_same_type_cast();
    both_different_type_cast();
}
fn arguments_type_cast() {
    TypeA.arguments(TA(0) as TA);
    assert!(true, "INFO: no need for cast: types specified in trait impl");

    TypeB.arguments(TB(0) as TB);
    assert!(true, "INFO: no need for cast: types specified in trait impl");

    // TypeA.arguments(32i32 as i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn results_type_cast() {
    let a = TypeA.results() as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: types specified in trait impl");

    let b = TypeB.results() as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: types specified in trait impl");

    // let x = TypeA.results() as Option<&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn both_same_type_cast() {
    let a = TypeA.both_same(TA(0)) as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: types specified in trait impl");

    let b = TypeB.both_same(TB(0)) as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: types specified in trait impl");

    // let x = TypeA.both_same(32i32) as Option<&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn both_different_type_cast() {
    let a = TypeA.both_different(TA(0)) as Option<&TC>;
    assert!(TC::is(a), "INFO: no need for cast: types specified in trait impl");

    let b = TypeB.both_different(TB(0)) as Option<&TC>;
    assert!(TC::is(b), "INFO: no need for cast: types specified in trait impl");

    // let x = TypeA.both_different(64u64) as Option<&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn turbofish() {
    turbofish_function();
    turbofish_type_as_trait();
    turbofish_type();
    turbofish_trait_params();
    turbofish_trait_bare();
}
fn turbofish_function() {
    // let a = TypeA.both_different::<T = TA, U = TC>(TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");
}
fn turbofish_type_as_trait() {
    // let a = <TypeA as MyTrait<T = TA, U = TC>>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");

    let a = <TypeA as MyTrait>::both_different(&TypeA, TA(0));
    assert!(TC::is(a), "INFO: types specified in trait impl");
}
fn turbofish_type() {
    let a = <TypeA>::both_different(&TypeA, TA(0));
    assert!(TC::is(a), "INFO: types specified in trait impl");

    let a = TypeA::both_different(&TypeA, TA(0));
    assert!(TC::is(a), "INFO: types specified in trait impl");
}
fn turbofish_trait_params() {
    let a = <dyn MyTrait<T = TA, U = TC>>::both_different(&TypeA, TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types: types specified in trait impl");

    // let a = <impl MyTrait<T = TA, U = TC>>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait<T = TA, U = TC>::both_different(&TypeA, TA(0));
    // let a = impl MyTrait<T = TA, U = TC>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: syntax error");

    // let a = MyTrait<T = TA, U = TC>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: syntax error");

    // let a = MyTrait::<T = TA, U = TC>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");
}
fn turbofish_trait_bare() {
    // let a = <dyn MyTrait>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: E0191: missing associated type value");

    // let a = <impl MyTrait>::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait::both_different(&TypeA, TA(0));
    // let a = impl MyTrait::both_different(&TypeA, TA(0));
    assert!(!false, "ERROR: syntax error");

    let a = MyTrait::both_different(&TypeA, TA(0));
    assert!(TC::is(a), "INFO: types specified in trait impl");
}
