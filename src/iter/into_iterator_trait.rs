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

// i32 + &i32 + &&i32
//    VO  [O  V&  [&
// O  x       x
// &  x   x   x   x
// i  x   x   x   x
// I  x   x   x   x
// S  x   -   x   -
// all but arrays
pub fn main() {
    call(" VO", vec![1i32, 2, 3]);
    call("&VO", &vec![1i32, 2, 3]);
    call("iVO", vec![1i32, 2, 3].iter());
    call("IVO", vec![1i32, 2, 3].into_iter());
    call("SVO", vec![1i32, 2, 3].as_slice());

    // call(" [O", [1i32, 2, 3]);
    call("&[O", &[1i32, 2, 3]);
    call("i[O", [1i32, 2, 3].iter());
    call("I[O", [1i32, 2, 3].into_iter());

    call(" V&", vec![&1i32, &2, &3]);
    call("&V&", &vec![&1i32, &2, &3]);
    call("iV&", vec![&1i32, &2, &3].iter());
    call("IV&", vec![&1i32, &2, &3].into_iter());
    call("SV&", vec![&1i32, &2, &3].as_slice());

    // call(" [&", [&1i32, &2, &3]);
    call("&[&", &[&1i32, &2, &3]);
    call("i[&", [&1i32, &2, &3].iter());
    call("I[&", [&1i32, &2, &3].into_iter());
}
// i32
//    VO  [O  V&  [&
// O  x
// &
// i
// I  x
// S
//
// &i32
//    VO  [O  V&  [&
// O          x
// &  x   x
// i  x   x
// I      x   x
// S  x
//
// &&i32
//    VO  [O  V&  [&
// O
// &          x   x
// i          x   x
// I              x
// S          x
