//! Example trait with type parameter as associate type with generic type parameters
use super::MyTrait;
use super::{Is, TA, TB, TC};
use std::any;
use std::marker::PhantomData;
// WARN: requires working around E0207
pub struct MyType<T, U> {
    _a: PhantomData<T>,
    _b: PhantomData<U>,
}
impl<T, U> MyType<T, U> {
    fn new() -> Self {
        MyType { _a: PhantomData, _b: PhantomData }
    }
}
impl<A: 'static, B: 'static> MyTrait for MyType<A, B> {
    type T = A;
    type U = B;
    fn arguments(&self, _arg: Self::T) {}
    fn results(&self) -> Option<&Self::T> {
        any_magic()
    }
    fn both_same(&self, _arg: Self::T) -> Option<&Self::T> {
        any_magic()
    }
    fn both_different<'a>(&self, _arg: Self::T) -> Option<&'a Self::U> {
        any_magic()
    }
}
fn any_magic<'a, T: 'static>() -> Option<&'a T> {
    // Some(&TA(1)) // ERROR: cannot cast to generic type
    // assert!(!false, "ERROR: E0308: mismatched types");
    let type_id = any::TypeId::of::<T>();
    if type_id == any::TypeId::of::<TA>() {
        Some(any::Any::downcast_ref::<T>(&TA(1)).unwrap())
    } else if type_id == any::TypeId::of::<TB>() {
        Some(any::Any::downcast_ref::<T>(&TB(2)).unwrap())
    } else if type_id == any::TypeId::of::<TC>() {
        Some(any::Any::downcast_ref::<T>(&TC(3)).unwrap())
    } else if type_id == any::TypeId::of::<i32>() {
        Some(any::Any::downcast_ref::<T>(&32i32).unwrap())
    } else {
        None
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
    // MyType::new().arguments(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // MyType::new().arguments(TB(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn results_type_elision() {
    // let a = MyType::new().results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b = MyType::new().results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_same_type_elision() {
    // let a = MyType::new().both_same(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b = MyType::new().both_same(TB(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_different_type_elision() {
    let aa = MyType::new().both_different(TA(0));
    let ab = MyType::new().both_different(TA(0));
    let ac = MyType::new().both_different(TA(0));
    let bc = MyType::new().both_different(TB(0));
    assert!(TA::is(aa), "INFO: type inferred from later usage");
    assert!(TB::is(ab), "INFO: type inferred from later usage");
    assert!(TC::is(ac), "INFO: type inferred from later usage");
    assert!(TC::is(bc), "INFO: type inferred from later usage");

    // let ax = MyType::new().both_different(TA(0));
    assert!(!false, "ERROR: E0282 type annotations needed - cannot infer type");

    let xx = MyType::new().both_different(64u64);
    assert!(i32::is(xx), "INFO: type inferred from later usage");
}
fn explicitly_typed() {
    arguments_explicitly_typed();
    results_explicitly_typed();
    both_same_explicitly_typed();
    both_different_explicitly_typed();
}
fn arguments_explicitly_typed() {
    // MyType::new().arguments(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // MyType::new().arguments(TB(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // MyType::new().arguments(32i32);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn results_explicitly_typed() {
    // let a: Option<&TA> = MyType::new().results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b: Option<&TB> = MyType::new().results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let x: Option<&i32> = MyType::new().results();
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_same_explicitly_typed() {
    // let a: Option<&TA> = MyType::new().both_same(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b: Option<&TB> = MyType::new().both_same(TB(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let x: Option<&i32> = MyType::new().both_same(32i32);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_different_explicitly_typed() {
    let aa: Option<&TA> = MyType::new().both_different(TA(0));
    assert!(TA::is(aa), "INFO: no need for explicit type: inferred from later usage");

    let ab: Option<&TB> = MyType::new().both_different(TA(0));
    assert!(TB::is(ab), "INFO: no need for explicit type: inferred from later usage");

    let ac: Option<&TC> = MyType::new().both_different(TA(0));
    assert!(TC::is(ac), "INFO: no need for explicit type: inferred from later usage");

    let bc: Option<&TC> = MyType::new().both_different(TA(0));
    assert!(TC::is(bc), "INFO: no need for explicit type: inferred from later usage");

    let xx: Option<&i32> = MyType::new().both_different(64u64);
    assert!(i32::is(xx), "INFO: no need for explicit type: inferred from later usage");
}
fn type_cast() {
    arguments_type_cast();
    results_type_cast();
    both_same_type_cast();
    both_different_type_cast();
}
fn arguments_type_cast() {
    // MyType::new().arguments(TA(0) as TA);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // MyType::new().arguments(TB(0) as TB);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // MyType::new().arguments(32i32 as i32);
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn results_type_cast() {
    // let a = MyType::new().results() as Option<&TA>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b = MyType::new().results() as Option<&TB>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let x = MyType::new().results() as Option<&i32>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_same_type_cast() {
    // let a = MyType::new().both_same(TA(0)) as Option<&TA>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let b = MyType::new().both_same(TB(0)) as Option<&TB>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");

    // let x = MyType::new().both_same(32i32) as Option<&i32>;
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn both_different_type_cast() {
    let aa = MyType::new().both_different(TA(0)) as Option<&TA>;
    assert!(TA::is(aa), "INFO: no need for cast: type inferred from later usage");

    let ab = MyType::new().both_different(TA(0)) as Option<&TB>;
    assert!(TB::is(ab), "INFO: no need for cast: type inferred from later usage");

    let ac = MyType::new().both_different(TA(0)) as Option<&TC>;
    assert!(TC::is(ac), "INFO: no need for cast: type inferred from later usage");

    let bc = MyType::new().both_different(TB(0)) as Option<&TC>;
    assert!(TC::is(bc), "INFO: no need for cast: type inferred from later usage");

    let xx = MyType::new().both_different(64u64) as Option<&i32>;
    assert!(i32::is(xx), "INFO: no need for cast: type inferred from later usage");
}
fn turbofish() {
    turbofish_function();
    turbofish_type_as_trait();
    turbofish_type();
    turbofish_trait_params();
    turbofish_trait_bare();
}
fn turbofish_function() {
    // let a = MyType::new().both_different::<T = TA, U = TC>(TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");
}
fn turbofish_type_as_trait() {
    // let a = <MyType<TA, TC> as MyTrait<T = TA, U = TC>>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");

    // let a = <MyType as MyTrait<T = TA, U = TC>>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0243: wrong number of type arguments");
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");

    let a = <MyType<TA, TC> as MyTrait>::both_different(&MyType::new(), TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types: type inferred from later usage");

    // let a = <MyType as MyTrait>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0243: wrong number of type arguments");
}
fn turbofish_type() {
    let a = <MyType<TA, TC>>::both_different(&MyType::new(), TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types: type inferred from later usage");

    // let a = MyType<TA, TC>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: syntax error");

    // let a = <MyType>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0243: wrong number of type arguments");

    let ab = MyType::both_different(&MyType::new(), TA(0));
    let ac = MyType::both_different(&MyType::new(), TA(0));
    // let a_ = MyType::both_different(&MyType::new(), TA(0));
    assert!(TB::is(ab), "INFO: no need for explicit types: type inferred from later usage");
    assert!(TC::is(ac), "INFO: no need for explicit types: type inferred from later usage");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
fn turbofish_trait_params() {
    let a = <dyn MyTrait<T = TA, U = TC>>::both_different(&MyType::new(), TA(0));
    assert!(TC::is(a), "INFO: no need for explicit types: type inferred from later usage");

    // let a = <impl MyTrait<T = TA, U = TC>>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait<T = TA, U = TC>::both_different(&MyType::new(), TA(0));
    // let a = impl MyTrait<T = TA, U = TC>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: syntax error");

    // let a = MyTrait<T = TA, U = TC>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: syntax error");

    // let a = MyTrait::<T = TA, U = TC>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0229: associated type bindings are not allowed here");
}
fn turbofish_trait_bare() {
    // let a = <dyn MyTrait>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0191: missing associated type value");

    // let a = <impl MyTrait>::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: E0562: `impl Trait` not allowed");

    // let a = dyn MyTrait::both_different(&MyType::new(), TA(0));
    // let a = impl MyTrait::both_different(&MyType::new(), TA(0));
    assert!(!false, "ERROR: syntax error");

    let ab = MyTrait::both_different(&MyType::new(), TA(0));
    let ac = MyTrait::both_different(&MyType::new(), TA(0));
    // let a_ = MyTrait::both_different(&MyType::new(), TA(0));
    assert!(TB::is(ab), "INFO: no need for explicit types: type inferred from later usage");
    assert!(TC::is(ac), "INFO: no need for explicit types: type inferred from later usage");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer other type");
}
