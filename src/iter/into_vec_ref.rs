//    O & i I S
// VO
// V& x       x
// [O
// [&
pub fn call<'a>(s: &str, items: impl Into<Vec<&'a i32>>) {
    println!("{}:{}:", module_path!(), s);
    for (i, x) in items.into().into_iter().enumerate() {
        super::print(i, *x);
    }
}
pub fn main() {
    // call("OVO", vec![1i32, 2, 3]);
    call("OV&", vec![&1i32, &2, &3]);
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
    call("SV&", vec![&1i32, &2, &3].as_slice());
}
