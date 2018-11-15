#[macro_export]
macro_rules! accepts_with {
    (@done [$($type:tt)*] [$($object:tt)*] |$var:pat| $expr:block) =>  {{
        let $var = new!($($object)*);
        let func = is_function!($($type)*);
        (func)($expr)
    }};
    (@done [$($type:tt)*] [$($object:tt)*] |$var:pat| $expr:expr) => {
        accepts_with!(@done [$($type)*] [$($object)*] |$var| { $expr });
    };
    (@done $($tts:tt)*) => {
        compile_error!("expected closure-like expression. e.x. |x| { x.to_vec() }");
    };
    (@object [$($type:tt)*] [$($object:tt)*] , $($tts:tt)*) => {
        accepts_with!(@done [$($type)*] [$($object)*] $($tts)*);
    };
    (@object [$($type:tt)*] [$($object:tt)*] $next:tt $($tts:tt)*) => {
        accepts_with!(@object [$($type)*] [$($object)* $next] $($tts)*);
    };
    (@object $($tts:tt)*) => {
        compile_error!("expected closure-like expression. e.x. |x| { x.to_vec() }");
    };
    (@type [$($type:tt)*] , $next:tt $($tts:tt)*) => {
        accepts_with!(@object [$($type)*] [$next] $($tts)*);
    };
    (@type [$($type:tt)*] $next:tt $($tts:tt)*) => {
        accepts_with!(@type [$($type)* $next] $($tts)*);
    };
    (@type $($tts:tt)*) => {
        compile_error!("expected two object types");
    };
    ($next:tt $($tts:tt)*) => {
        accepts_with!(@type [$next] $($tts)*);
    };
}
#[macro_export]
macro_rules! accepts {
    ($($tts:tt)*) => { accepts_with!($($tts)*, |x| { x }); };
}
#[macro_export]
macro_rules! accepts_borrow {
    ($($tts:tt)*) => { accepts_with!($($tts)*, |x| { &x }); };
}
#[macro_export]
macro_rules! new {
    ([T; N]) => { X::array_owned() };
    ([&T; N]) => { X::array_borrowed() };
    (&[T; N]) => { &X::array_owned() };
    (&[&T; N]) => { &X::array_borrowed() };
    (&[T]) => { &X::array_owned()[..] };
    (&[&T]) => { &X::array_borrowed()[..] };
    (Vec<T>) => { X::vec_owned() };
    (Vec<&T>) => { X::vec_borrowed() };
    (&Vec<T>) => { &X::vec_owned() };
    (&Vec<&T>) => { &X::vec_borrowed() };
    (Iterator<T>) => { new!(Vec<T>).into_iter() };
    (Iterator<&T>) => { new!(Vec<&T>).into_iter() };
    (&mut Iterator<T>) => { &mut new!(Iterator<T>) };
    (&mut Iterator<&T>) => { &mut new!(Iterator<&T>) };
    (IntoIterator<T>) => { XIntoIteratorOwned::new() };
    (IntoIterator<&T>) => { XIntoIteratorBorrowed::new() };
}
#[macro_export]
macro_rules! is_function {
    ([T; N]) => { is_array_owned__items_owned };
    ([&T; N]) => { is_array_owned__items_borrowed };
    (&[T; N]) => { is_array_borrowed__items_owned };
    (&[&T; N]) => { is_array_borrowed__items_borrowed };
    (&[T]) => { is_slice__items_owned };
    (&[&T]) => { is_slice__items_borrowed };
    (Vec<T>) => { is_vec_owned__items_owned };
    (Vec<&T>) => { is_vec_owned__items_borrowed };
    (&Vec<T>) => { is_vec_borrowed__items_owned };
    (&Vec<&T>) => { is_vec_borrowed__items_borrowed };
    (Iterator<T>) => { is_iterator_owned__items_owned };
    (Iterator<&T>) => { is_iterator_owned__items_borrowed };
    (&mut Iterator<T>) => { is_iterator_borrowed__items_owned };
    (&mut Iterator<&T>) => { is_iterator_borrowed__items_borrowed };
    (IntoIterator<T>) => { is_into_iterator__items_owned };
    (IntoIterator<&T>) => { is_into_iterator__items_borrowed };
    (IntoIterator<Borrow<T>>) => { is_into_iterator__borrow_items_owned };
    (IntoIterator<Borrow<&T>>) => { is_into_iterator__borrow_items_borrowed };
    (IntoIterator<Deref<T>>) => { is_into_iterator__deref_items_owned };
    (IntoIterator<Deref<&T>>) => { is_into_iterator__deref_items_borrowed };
    (IntoIterator<TraitAll>) => { is_into_iterator__trait_all };
    (IntoIterator<TraitOwned>) => { is_into_iterator__trait_owned };
    (IntoIterator<TraitBorrowed>) => { is_into_iterator__trait_borrowed };
    (IntoIterator<TraitBorrowedBorrowed>) => { is_into_iterator__trait_borrowed_borrowed };
}
