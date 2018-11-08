#![allow(dead_code)]
extern crate rust;
fn main() {
    rust::traits::main();
    // rust::iter::main();
}

// pub trait WatTrait<T, U> {
//     fn f1(&self, arg: T);
//     fn f2(&self, arg: U);
// }
// struct WatType(());
// impl<T, U> WatTrait<T, U> for WatType {
//     fn f1(&self, _arg: T) {}
//     fn f2(&self, _arg: U) {}
// }
// fn wat() {
//     let x = WatType(());
//     x.f1(32i32);
//     x.f2(64u64);
// }
