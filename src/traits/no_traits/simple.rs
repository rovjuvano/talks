//! Example types without using traits (inherent methods only) - basic implementation
use super::{Is, TA, TB, TC};
pub struct TypeA;
impl TypeA {
    pub fn arguments(&self, _arg: TA) {}
    pub fn results(&self) -> Option<&TA> {
        Some(&TA(1))
    }
    pub fn both_same(&self, _arg: TA) -> Option<&TA> {
        Some(&TA(1))
    }
    pub fn both_different(&self, _arg: TA) -> Option<&TC> {
        Some(&TC(3))
    }
}
pub struct TypeB;
impl TypeB {
    pub fn arguments(&self, _arg: TB) {}
    pub fn results(&self) -> Option<&TB> {
        Some(&TB(2))
    }
    pub fn both_same(&self, _arg: TB) -> Option<&TB> {
        Some(&TB(2))
    }
    pub fn both_different(&self, _arg: TB) -> Option<&TC> {
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
    assert!(true, "INFO: function declares concrete type and type always included with argument");

    TypeB.arguments(TB(0));
    assert!(true, "INFO: function declares concrete type and type always included with argument");
}
fn results_type_elision() {
    let a = TypeA.results();
    assert!(TA::is(a), "INFO: function declares concrete type");

    let b = TypeB.results();
    assert!(TB::is(b), "INFO: function declares concrete type");
}
fn both_same_type_elision() {
    let a = TypeA.both_same(TA(0));
    assert!(TA::is(a), "INFO: function declares concrete type");

    let b = TypeB.both_same(TB(0));
    assert!(TB::is(b), "INFO: function declares concrete type");
}
fn both_different_type_elision() {
    let a = TypeA.both_different(TA(0));
    assert!(TC::is(a), "INFO: function declares concrete type");

    let b = TypeB.both_different(TB(0));
    assert!(TC::is(b), "INFO: function declares concrete type");
}
fn explicitly_typed() {
    arguments_explicitly_typed();
    results_explicitly_typed();
    both_same_explicitly_typed();
    both_different_explicitly_typed();
}
fn arguments_explicitly_typed() {
    TypeA.arguments(TA(0));
    assert!(true, "INFO: no need for explicit type: function declares concrete type and type always included with argument");

    TypeB.arguments(TB(0));
    assert!(true, "INFO: no need for explicit type: function declares concrete type and type always included with argument");

    // TypeA.arguments(32i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn results_explicitly_typed() {
    let a: Option<&TA> = TypeA.results();
    assert!(TA::is(a), "INFO: no need for explicit type: function declares concrete type");

    let b: Option<&TB> = TypeB.results();
    assert!(TB::is(b), "INFO: no need for explicit type: function declares concrete type");

    // let a: Option<&i32> = TypeA.results();
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn both_same_explicitly_typed() {
    let a: Option<&TA> = TypeA.both_same(TA(0));
    assert!(TA::is(a), "INFO: no need for explicit type: function declares concrete type");

    let b: Option<&TB> = TypeB.both_same(TB(0));
    assert!(TB::is(b), "INFO: no need for explicit type: function declares concrete type");

    // let a: Option<&i32> = TypeA.both_same(32i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn both_different_explicitly_typed() {
    let a: Option<&TC> = TypeA.both_different(TA(0));
    assert!(TC::is(a), "INFO: no need for explicit type: function declares concrete type");

    let b: Option<&TC> = TypeB.both_different(TB(0));
    assert!(TC::is(b), "INFO: no need for explicit type: function declares concrete type");

    // let a: Option<&i32> = TypeA.both_different(64u64);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn type_cast() {
    arguments_type_cast();
    results_type_cast();
    both_same_type_cast();
    both_different_type_cast();
}
fn arguments_type_cast() {
    assert!(true, "INFO: no need for cast: function declares concrete type, type always included with argument, and casting to same type");
    TypeA.arguments(TA(0) as TA);

    TypeB.arguments(TB(0) as TB);
    assert!(true, "INFO: no need for cast: function declares concrete type, type always included with argument, and casting to same type");

    // TypeA.arguments(32i32 as i32);
    assert!(!false, "ERROR: E0308: mismatched types");
}
fn results_type_cast() {
    let a = TypeA.results() as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: function declares concrete type");

    let b = TypeB.results() as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: function declares concrete type");

    // let x = TypeA.results() as Option<&&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn both_same_type_cast() {
    let a = TypeA.both_same(TA(0)) as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: function declares concrete type");

    let b = TypeB.both_same(TB(0)) as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: function declares concrete type");

    // let x = TypeA.both_same(32i32) as Option<&&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn both_different_type_cast() {
    let a = TypeA.both_different(TA(0)) as Option<&TC>;
    assert!(TC::is(a), "INFO: no need for cast: function declares concrete type");

    let b = TypeB.both_different(TB(0)) as Option<&TC>;
    assert!(TC::is(b), "INFO: no need for cast: function declares concrete type");

    // let x = TypeA.both_different(64u64) as Option<&&i32>;
    assert!(!false, "ERROR: E0605: non-primitive cast");
}
fn turbofish() {
    // let a = TypeA.both_different::<TC, TA>(TC(0));
    // let a = TypeA::both_different::<TC, TA>(&TypeA, TC(0));
    assert!(!false, "ERROR: E0087: too many type parameters provided");
}
