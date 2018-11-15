//    O & i I S
// VO
// V&
// [O
// [&
// ERROR: does not compile: into_iter takes ownership but borrow produces reference
// use std::borrow::Borrow;
// pub fn call<'a, I>(s: &str, items: impl Borrow<I>)
// where
//     I: IntoIterator,
//     I::Item: Borrow<i32>,
// {
//     println!("{}:{}:", module_path!(), s);
//     for (i, x) in items.borrow().into_iter().enumerate() {
//         super::print(i, *x.borrow());
//     }
// }
// pub fn call<'a, I>(s: &str, items: &dyn Borrow<I>)
// where
//     I: IntoIterator,
//     I::Item: Borrow<i32>,
// {
//     println!("{}:{}:", module_path!(), s);
//     for (i, x) in items.borrow().into_iter().enumerate() {
//         super::print(i, *x.borrow());
//     }
// }
pub fn main() {
    // call("OVO", vec![1i32, 2, 3]);
    // call("OV&", vec![&1i32, &2, &3]);
    // call("O[O", [1i32, 2, 3]);
    // call("O[&", [&1i32, &2, &3]);

    // call("&VO", &vec![1i32, 2, 3]);
    // call("&V&", &vec![&1i32, &2, &3]);
    // call("&[O", &[1i32, 2, 3]);
    // call("&[&", &[&1i32, &2, &3]);

    // call("iVO", vec![1i32, 2, 3].iter());
    // call("iV&", vec![&1i32, &2, &3].iter());
    // call("i[O", [1i32, 2, 3].iter());
    // call("i[&", [&1i32, &2, &3].iter());

    // call("IVO", vec![1i32, 2, 3].into_iter());
    // call("IV&", vec![&1i32, &2, &3].into_iter());
    // call("I[O", [1i32, 2, 3].into_iter());
    // call("I[&", [&1i32, &2, &3].into_iter());

    // call("SVO", vec![1i32, 2, 3].as_slice());
    // call("SV&", vec![&1i32, &2, &3].as_slice());
}
