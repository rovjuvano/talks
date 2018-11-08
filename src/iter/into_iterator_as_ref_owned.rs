#[allow(dead_code)]
pub fn call<I>(s: &str, items: I)
where
    I: IntoIterator,
    I::Item: AsRef<i32>,
{
    println!("{}:{}:", module_path!(), s);
    for (i, x) in items.into_iter().enumerate() {
        super::print(i, *x.as_ref());
    }
}
//    VO  [O  V&  [&
// O
// &
// i
// I
// S
// nothing!!!!
pub fn main() {
    // call(" VO", vec![1i32, 2, 3]);
    // call("&VO", &vec![1i32, 2, 3]);
    // call("iVO", vec![1i32, 2, 3].iter());
    // call("IVO", vec![1i32, 2, 3].into_iter());
    // call("SVO", vec![1i32, 2, 3].as_slice());

    // call(" [O", [1i32, 2, 3]);
    // call("&[O", &[1i32, 2, 3]);
    // call("i[O", [1i32, 2, 3].iter());
    // call("I[O", [1i32, 2, 3].into_iter());

    // call(" V&", vec![&1i32, &2, &3]);
    // call("&V&", &vec![&1i32, &2, &3]);
    // call("iV&", vec![&1i32, &2, &3].iter());
    // call("IV&", vec![&1i32, &2, &3].into_iter());
    // call("SV&", vec![&1i32, &2, &3].as_slice());

    // call(" [&", [&1i32, &2, &3]);
    // call("&[&", &[&1i32, &2, &3]);
    // call("i[&", [&1i32, &2, &3].iter());
    // call("I[&", [&1i32, &2, &3].into_iter());
}
