//    O & i I S
// VO
// V&
// [O
// [&
// ERROR: does not compile: into_iter takes ownership but as_ref produces reference
// pub fn call<I>(s: &str, items: &dyn AsRef<I>)
// where
//     I: IntoIterator<Item = i32>,
// {
//     println!("{}:{}:", module_path!(), s);
//     for (i, x) in items.as_ref().into_iter().enumerate() {
//         super::print(i, x);
//     }
// }
// does not compile: into_iter takes ownership but as_ref produces reference
// pub fn call6<I>(s: &str, items: impl AsRef<I>)
// where
//     I: IntoIterator<Item = i32>,
// {
//     println!("{}:{}:", module_path!(), s);
//     for (i, x) in items.as_ref().into_iter().enumerate() {
//         super::print(i, x);
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
