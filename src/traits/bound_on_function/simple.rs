//! Example trait with type parameter on function with basic implementation
use super::MyTrait;
use super::{Is, TA, TB, TC};
use std::any;
struct MyType;
impl MyTrait for MyType {
    fn arguments<T: 'static>(&self, _arg: T) {}
    fn results<T: 'static>(&self) -> Option<&T> {
        // Some(&TA(1)) // ERROR: cannot cast to generic type
        // assert!(!false, "ERROR: E0308: mismatched types");
        // WARN: any requires T to be 'static
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
    fn both_same<T: 'static>(&self, _arg: T) -> Option<&T> {
        self.results()
    }
    fn both_different<T, U: 'static>(&self, _arg: T) -> Option<&U> {
        self.results()
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
    MyType.arguments(TA(0));
    assert!(true, "INFO: type always included with value");
}
fn results_type_elision() {
    let a = MyType.results();
    let b = MyType.results();
    // let x = MyType.results();
    assert!(TA::is(a), "INFO: type inferred from later usage");
    assert!(TB::is(b), "INFO: type inferred from later usage");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer type");
}
fn both_same_type_elision() {
    let a = MyType.both_same(TA(0));
    let b = MyType.both_same(TB(0));
    let x = MyType.both_same(TA(0));
    assert!(TA::is(a), "INFO: type inferred from argument");
    assert!(TB::is(b), "INFO: type inferred from argument");
    assert!(TA::is(x), "INFO: type inferred from argument");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer type");
}
fn both_different_type_elision() {
    let aa = MyType.both_different(TA(0));
    let ab = MyType.both_different(TA(0));
    let ac = MyType.both_different(TA(0));
    let bc = MyType.both_different(TB(0));
    // let xx = MyType.both_different(TA(0));
    assert!(TA::is(aa), "INFO: type inferred from later usage");
    assert!(TB::is(ab), "INFO: type inferred from later usage");
    assert!(TA::is(ac), "INFO: type inferred from later usage");
    assert!(TA::is(bc), "INFO: type inferred from later usage");
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer type");
}
fn explicitly_typed() {
    arguments_explicitly_typed();
    results_explicitly_typed();
    both_same_explicitly_typed();
    both_different_explicitly_typed();
}
fn arguments_explicitly_typed() {
    MyType.arguments(TA(0));
    assert!(true, "INFO: no need for explicit type: type always included with argument");

    MyType.arguments(32i32);
    assert!(true, "INFO: no need for explicit type: type always included with argument");
}
fn results_explicitly_typed() {
    let a: Option<&TA> = MyType.results();
    assert!(TA::is(a), "INFO: no need for explicit type: inferred from later usage");

    let x: Option<&i32> = MyType.results();
    assert!(i32::is(x), "INFO: no need for explicit type: inferred from later usage");
}
fn both_same_explicitly_typed() {
    let a: Option<&TA> = MyType.both_same(TA(0));
    assert!(TA::is(a), "INFO: no need for explicit type: inferred from argument");

    let x: Option<&i32> = MyType.both_same(32i32);
    assert!(i32::is(x), "INFO: no need for explicit type: inferred from argument");
}
fn both_different_explicitly_typed() {
    let aa: Option<&TA> = MyType.both_different(TA(0));
    assert!(TA::is(aa), "INFO: no need for explicit type: inferred from later usage");

    let ab: Option<&TB> = MyType.both_different(TA(0));
    assert!(TB::is(ab), "INFO: no need for explicit type: inferred from later usage");

    let ac: Option<&TC> = MyType.both_different(TA(0));
    assert!(TC::is(ac), "INFO: no need for explicit type: inferred from later usage");

    let bc: Option<&TC> = MyType.both_different(TB(0));
    assert!(TC::is(bc), "INFO: no need for explicit type: inferred from later usage");

    let x: Option<&i32> = MyType.both_different(64u64);
    assert!(i32::is(x), "INFO: no need for explicit type: inferred from later usage");
}
fn type_cast() {
    arguments_type_cast();
    results_type_cast();
    both_same_type_cast();
    both_different_type_cast();
}
fn arguments_type_cast() {
    MyType.arguments(TA(0) as TA);
    assert!(true, "INFO: no need for cast: type always included with argument");

    MyType.arguments(32i32 as i32);
    assert!(true, "INFO: no need for cast: type always included with argument");
}
fn results_type_cast() {
    let a = MyType.results() as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: type inferred from later usage");

    let x = MyType.results() as Option<&i32>;
    assert!(i32::is(x), "INFO: no need for cast: type inferred from later usage");
}
fn both_same_type_cast() {
    let a = MyType.both_same(TA(0)) as Option<&TA>;
    assert!(TA::is(a), "INFO: no need for cast: type inferred from argument");

    let x = MyType.both_same(32i32) as Option<&i32>;
    assert!(i32::is(x), "INFO: no need for cast: type inferred from argument");
}
fn both_different_type_cast() {
    let aa = MyType.both_different(TA(0)) as Option<&TA>;
    assert!(TA::is(aa), "INFO: no need for cast: inferred from later usage");

    let ab = MyType.both_different(TA(0)) as Option<&TB>;
    assert!(TB::is(ab), "INFO: no need for cast: inferred from later usage");

    let ac = MyType.both_different(TA(0)) as Option<&TC>;
    assert!(TC::is(ac), "INFO: no need for cast: inferred from later usage");

    let bc = MyType.both_different(TB(0)) as Option<&TC>;
    assert!(TC::is(bc), "INFO: no need for cast: inferred from later usage");

    let x = MyType.both_different(64u64) as Option<&i32>;
    assert!(i32::is(x), "INFO: no need for cast: inferred from later usage");
}
fn turbofish() {
    arguments_turbofish();
    results_turbofish();
    both_same_turbofish();
    both_different_turbofish();
}
fn arguments_turbofish() {
    MyType.arguments::<TA>(TA(0));
    assert!(true, "INFO: no need for turbofish: type always included with argument");

    MyType.arguments::<i32>(32i32);
    assert!(true, "INFO: no need for turbofish: type always included with argument");
}
fn results_turbofish() {
    let a = MyType.results::<TA>();
    assert!(TA::is(a), "INFO: no need for turbofish: type inferred from later usage");

    let x = MyType.results::<i32>();
    assert!(i32::is(x), "INFO: no need for turbofish: type inferred from later usage");
}
fn both_same_turbofish() {
    let a = MyType.both_same::<TA>(TA(0));
    assert!(TA::is(a), "INFO: no need for turbofish: type inferred from argument");

    let x = MyType.both_same::<i32>(32i32);
    assert!(i32::is(x), "INFO: no need for turbofish: type inferred from argument");
}
fn both_different_turbofish() {
    let aa = MyType.both_different::<TA, TA>(TA(0));
    assert!(TA::is(aa), "INFO: no need for turbofish: inferred from later usage");

    let ab = MyType.both_different::<TA, TB>(TA(0));
    assert!(TB::is(ab), "INFO: no need for turbofish: inferred from later usage");

    let ac = MyType.both_different::<TA, TC>(TA(0));
    assert!(TC::is(ac), "INFO: no need for turbofish: inferred from later usage");

    let bc = MyType.both_different::<TB, TC>(TB(0));
    assert!(TC::is(bc), "INFO: no need for turbofish: inferred from later usage");

    let x = MyType.both_different::<u64, i32>(64u64);
    assert!(i32::is(x), "INFO: no need for turbofish: inferred from later usage");

    // let a: Option<&TC> = MyType.both_different::<TC>(TA(0)) as Option<&TC>;
    assert!(!false, "ERROR: either none or all types must be provided");

    #[allow(unused_variables)]
    let xc = MyType.both_different::<_, TC>(TA(0));
    assert!(true, "INFO: placeholder may be used for inferable type arguments");

    // let ax = MyType.both_different::<TA, _>(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer type");

    // let xx = MyType.both_different::<_, _>(TA(0));
    assert!(!false, "ERROR: E0282: type annotations needed: cannot infer type");

    let xx = MyType.both_different::<_, _>(TA(0));
    assert!(TC::is(xx), "INFO: placeholder may be used for inferable type arguments");
}
