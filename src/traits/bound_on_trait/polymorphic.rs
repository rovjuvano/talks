//! Example trait with type parameter on trait with multiple implementations
use super::MyTrait;
use super::{Is, TA, TB, TC};
struct MyType;
impl MyTrait<TA, TC> for MyType {
    fn arguments(&self, _arg: TA) {}
    fn results(&self) -> Option<&TA> {
        Some(&TA(1))
    }
    fn both_same(&self, _arg: TA) -> Option<&TA> {
        Some(&TA(1))
    }
    fn both_different(&self, _arg: TA) -> Option<&TC> {
        Some(&TC(3))
    }
}
impl MyTrait<TA, TB> for MyType {
    fn arguments(&self, _arg: TA) {}
    fn results(&self) -> Option<&TA> {
        Some(&TA(1))
    }
    fn both_same(&self, _arg: TA) -> Option<&TA> {
        Some(&TA(1))
    }
    fn both_different(&self, _arg: TA) -> Option<&TB> {
        Some(&TB(2))
    }
}
impl MyTrait<TB, TC> for MyType {
    fn arguments(&self, _arg: TB) {}
    fn results(&self) -> Option<&TB> {
        Some(&TB(2))
    }
    fn both_same(&self, _arg: TB) -> Option<&TB> {
        Some(&TB(2))
    }
    fn both_different(&self, _arg: TB) -> Option<&TC> {
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
    // MyType.arguments(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    MyType.arguments(TB(0));
    assert!(true, "INFO: non-ambiguous - only one trait impl with inferred type");
}
fn results_type_elision() {
    // let a = MyType.results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b = MyType.results();
    assert!(TB::is(b), "INFO: type inferred to single impl from later usage");
}
fn both_same_type_elision() {
    // let a = MyType.both_same(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b = MyType.both_same(TB(0));
    assert!(TB::is(b), "INFO: type inferred to single impl from argument");
}
fn both_different_type_elision() {
    // let aa = MyType.both_different(TA(0));
    // TA::is(aa);
    assert!(!false, "ERROR: E0277: trait not implemented for types");

    let ab = MyType.both_different(TA(0));
    let ac = MyType.both_different(TA(0));
    assert!(TB::is(ab), "INFO: type inferred to single impl from later usage");
    assert!(TC::is(ac), "INFO: type inferred to single impl from later usage");

    let bc = MyType.both_different(TB(0));
    assert!(TC::is(bc), "INFO: non-ambiguous - only one trait impl for type of argument");

    // let x = MyType.both_different(64u64);
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn explicitly_typed() {
    arguments_explicitly_typed();
    results_explicitly_typed();
    both_same_explicitly_typed();
    both_different_explicitly_typed();
}
fn arguments_explicitly_typed() {
    // MyType.arguments(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    MyType.arguments(TB(0));
    assert!(true, "INFO: no need for explicit type: type always included with argument");

    // MyType.arguments(32i32);
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn results_explicitly_typed() {
    // let a: Option<&TA> = MyType.results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b: Option<&TB> = MyType.results();
    assert!(TB::is(b), "INFO: no need for explicit type: inferred from later usage");

    // let x: Option<&i32> = MyType.results();
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn both_same_explicitly_typed() {
    // let a: Option<&TA> = MyType.both_same(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b: Option<&TB> = MyType.both_same(TB(0));
    assert!(TB::is(b), "INFO: no need for explicit type: inferred from argument");

    // let x: Option<&i32> = MyType.both_same(32i32);
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn both_different_explicitly_typed() {
    // let aa: Option<&TA> = MyType.both_different(TA(0));
    assert!(!false, "ERROR: E0277: trait not implemented for types");

    let ab: Option<&TB> = MyType.both_different(TA(0));
    assert!(TB::is(ab), "INFO: no need for explicit type: inferred from later usage");

    let ac: Option<&TC> = MyType.both_different(TA(0));
    assert!(TC::is(ac), "INFO: no need for explicit type: inferred from later usage");

    let bc: Option<&TC> = MyType.both_different(TA(0));
    assert!(TC::is(bc), "INFO: no need for explicit type: inferred from later usage");

    // let x: Option<&i32> = MyType.both_different(64u64);
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn type_cast() {
    arguments_type_cast();
    results_type_cast();
    both_same_type_cast();
    both_different_type_cast();
}
fn arguments_type_cast() {
    // MyType.arguments(TA(0) as TA);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    MyType.arguments(TB(0) as TB);
    assert!(true, "INFO: no need for cast: type always included with argument");

    // MyType.arguments(32i32 as i32);
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn results_type_cast() {
    // let a = MyType.results() as Option<&TA>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b = MyType.results() as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: type inferred from later usage");

    // let x = MyType.results() as Option<&i32>;
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn both_same_type_cast() {
    // let a = MyType.both_same(TA(0)) as Option<&TA>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let b = MyType.both_same(TB(0)) as Option<&TB>;
    assert!(TB::is(b), "INFO: no need for cast: type inferred from argument");

    // let x = MyType.both_same(32i32) as Option<&i32>;
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn both_different_type_cast() {
    // let aa = MyType.both_different(TA(0)) as Option<&TA>;
    assert!(!false, "ERROR: E0277: trait not implemented for types");

    let ab = MyType.both_different(TA(0)) as Option<&TB>;
    assert!(TB::is(ab), "INFO: no need for cast: type inferred from later usage");

    let ac = MyType.both_different(TA(0)) as Option<&TC>;
    assert!(TC::is(ac), "INFO: no need for cast: type inferred from later usage");

    let bc = MyType.both_different(TB(0)) as Option<&TC>;
    assert!(TC::is(bc), "INFO: no need for cast: type inferred from later usage");

    // let x = MyType.both_different(64u64) as Option<&i32>;
    assert!(!false, "ERROR: E0277: trait not implemented for types");
}
fn turbofish() {
    turbofish_function();
    turbofish_type_as_trait();
    turbofish_type();
    turbofish_trait_params();
    turbofish_trait_bare();
}
fn turbofish_function() {
    // let a = MyType.both_different::<TA, TC>(TA(0));
    assert!(!false, "ERROR: E0087: too many type parameters provided");
}
fn turbofish_type_as_trait() {
    let a = <MyType as MyTrait<TA, TC>>::both_different(&MyType, TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types - ambiguity resolved by later use");

    // let a = <MyType as MyTrait>::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: E0089: too few type parameters provided");
}
fn turbofish_type() {
    let ab = <MyType>::both_different(&MyType, TA(0));
    let ac = <MyType>::both_different(&MyType, TA(0));
    // let a_ = <MyType>::both_different(&MyType, TA(0));
    assert!(TB::is(ab), "INFO: ambiguity resolved by later use");
    assert!(TC::is(ac), "INFO: ambiguity resolved by later use");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    let ab = MyType::both_different(&MyType, TA(0));
    let ac = MyType::both_different(&MyType, TA(0));
    // let a_ = MyType::both_different(&MyType, TA(0));
    assert!(TB::is(ab), "INFO: ambiguity resolved by later use");
    assert!(TC::is(ac), "INFO: ambiguity resolved by later use");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn turbofish_trait_params() {
    let a = <dyn MyTrait<TA, TC>>::both_different(&MyType, TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types - ambiguity resolved by later use");

    // <impl MyTrait<TA, TC>>::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait<TA, TC>::both_different(&MyType, TA(0));
    // let a = impl MyTrait<TA, TC>::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: syntax error");

    let a = MyTrait::<TA, TC>::both_different(&MyType, TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types - ambiguity resolved by later use");
}
fn turbofish_trait_bare() {
    // let a = <dyn MyTrait>::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: E0243: wrong number of type arguments");

    // let a = <impl MyTrait>::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait::both_different(&MyType, TA(0));
    // let a = impl MyTrait::both_different(&MyType, TA(0));
    assert!(!false, "ERROR: syntax error");

    let ab = MyTrait::both_different(&MyType, TA(0));
    let ac = MyTrait::both_different(&MyType, TA(0));
    // let a_ = MyTrait::both_different(&MyType, TA(0));
    assert!(TB::is(ab), "INFO: ambiguity resolved by later use");
    assert!(TC::is(ac), "INFO: ambiguity resolved by later use");
    assert!(!false, "ERROR: E0282 type annotations needed - cannot infer type");
}
