//    O & i I S
// VO x x x x x
// V& x x x x x
// [O   x x x
// [&   x x x
// NOTE: all but arrays
// # for just i32
//    O & i I S
// VO x     x
// V&
// [O
// [&
// # for just &i32
//    O & i I S
// VO   x x   x
// V& x     x
// [O   x x x
// [&
// # for just &&i32
//    O & i I S
// VO
// V&   x x   x
// [O
// [&   x x x
pub fn call<I>(s: &str, items: I)
where
    I: IntoIterator,
    I::Item: MyMagicTrait,
{
    println!("{}:{}:", module_path!(), s);
    for (i, x) in items.into_iter().enumerate() {
        super::print(i, x.to_i32());
    }
}
pub trait MyMagicTrait {
    fn to_i32(&self) -> i32;
}
impl MyMagicTrait for i32 {
    fn to_i32(&self) -> i32 {
        *self
    }
}
impl<'a> MyMagicTrait for &'a i32 {
    fn to_i32(&self) -> i32 {
        **self
    }
}
impl<'a, 'b> MyMagicTrait for &'b &'a i32 {
    fn to_i32(&self) -> i32 {
        ***self
    }
}
pub fn main() {
    call("OVO", vec![1i32, 2, 3]);
    call("OV&", vec![&1i32, &2, &3]);
    // call("O[O", [1i32, 2, 3]);
    // call("O[&", [&1i32, &2, &3]);

    call("&VO", &vec![1i32, 2, 3]);
    call("&V&", &vec![&1i32, &2, &3]);
    call("&[O", &[1i32, 2, 3]);
    call("&[&", &[&1i32, &2, &3]);

    call("iVO", vec![1i32, 2, 3].iter());
    call("iV&", vec![&1i32, &2, &3].iter());
    call("i[O", [1i32, 2, 3].iter());
    call("i[&", [&1i32, &2, &3].iter());

    call("IVO", vec![1i32, 2, 3].into_iter());
    call("IV&", vec![&1i32, &2, &3].into_iter());
    call("I[O", [1i32, 2, 3].into_iter());
    call("I[&", [&1i32, &2, &3].into_iter());

    call("SVO", vec![1i32, 2, 3].as_slice());
    call("SV&", vec![&1i32, &2, &3].as_slice());
}
