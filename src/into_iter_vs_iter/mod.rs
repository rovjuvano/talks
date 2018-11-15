#![allow(non_snake_case)]
#![allow(unused)]
use std::slice::Iter;
use std::vec::IntoIter;
pub fn main() {
    array::main();
    slice::main();
    vec::main();
}
pub mod array {
    use std::slice::Iter;
    pub fn main() {
        array__owned_items();
        array__borrowed_items();
    }
    fn array__owned_items() {
        let x: [i32; 3] = [1i32, 2, 3];
        into_iter__array_owned__owned_items__iterator_ref(x.clone());
        into_iter__array_borrowed__owned_items__iterator_ref(&x.clone());
        iter__array_owned__owned_items__iterator_ref(x.clone());
        iter__array_borrowed__owned_items__iterator_ref(&x.clone());
    }
    fn array__borrowed_items() {
        let x: [&i32; 3] = [&1i32, &2, &3];
        into_iter__array_owned__borrowed_items__iterator_ref_ref(x.clone());
        into_iter__array_borrowed__borrowed_items__iterator_ref_ref(&x.clone());
        iter__array_owned__borrowed_items__iterator_ref_ref(x.clone());
        iter__array_borrowed__borrowed_items__iterator_ref_ref(&x.clone());
    }
    fn into_iter__array_owned__owned_items__iterator_ref<T>(x: [T; 3]) {
        let iter: Iter<T> = x.into_iter(); // not into
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn into_iter__array_owned__borrowed_items__iterator_ref_ref<T>(x: [&T; 3]) {
        let iter: Iter<&T> = x.into_iter(); // not into but often practically equivalent
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn into_iter__array_borrowed__owned_items__iterator_ref<T>(x: &[T; 3]) {
        let iter: Iter<T> = x.into_iter(); // not into
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn into_iter__array_borrowed__borrowed_items__iterator_ref_ref<T>(x: &[&T; 3]) {
        let iter: Iter<&T> = x.into_iter(); // not into but often practically equivalent
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn iter__array_owned__owned_items__iterator_ref<T>(x: [T; 3]) {
        let iter: Iter<T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn iter__array_owned__borrowed_items__iterator_ref_ref<T>(x: [&T; 3]) {
        let iter: Iter<&T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn iter__array_borrowed__owned_items__iterator_ref<T>(x: &[T; 3]) {
        let iter: Iter<T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn iter__array_borrowed__borrowed_items__iterator_ref_ref<T>(x: &[&T; 3]) {
        let iter: Iter<&T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
}
mod slice {
    use std::slice::Iter;
    pub fn main() {
        slice__borrowed_items();
        slice__owned_items();
    }
    fn slice__owned_items() {
        // let x = vec![1i32, 2, 3]; let x: &[i32] = x.as_slice();
        // let x: &[i32] = &vec![1i32, 2, 3];
        let x: &[i32] = &[1i32, 2, 3];
        into_iter__slice__owned_items__iterator_ref(x);
        iter__slice__owned_items__iterator_ref(x);
    }
    fn slice__borrowed_items() {
        // let x = vec![&1i32, &2, &3]; let x: &[&i32] = x.as_slice();
        // let x: &[&i32] = &vec![&1i32, &2, &3];
        let x: &[&i32] = &[&1i32, &2, &3];
        into_iter__slice__borrowed_items__iterator_ref_ref(x);
        iter__slice__borrowed_items__iterator_ref_ref(x);
    }
    fn into_iter__slice__owned_items__iterator_ref<T>(x: &[T]) {
        let iter: Iter<T> = x.into_iter(); // not into
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn into_iter__slice__borrowed_items__iterator_ref_ref<T>(x: &[&T]) {
        let iter: Iter<&T> = x.into_iter(); // not into but often practically equivalent
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn iter__slice__owned_items__iterator_ref<T>(x: &[T]) {
        let iter: Iter<T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn iter__slice__borrowed_items__iterator_ref_ref<T>(x: &[&T]) {
        let iter: Iter<&T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
}
pub mod vec {
    use std::slice::Iter;
    use std::vec::IntoIter;
    pub fn main() {
        vec__owned_items();
        vec__borrowed_items();
    }
    fn vec__owned_items() {
        let x: Vec<i32> = vec![1i32, 2, 3];
        into_iter__vec_owned__owned_items__iterator_owned(x.clone());
        into_iter__vec_borrowed__owned_items__iterator_ref(&x.clone());
        iter__vec_owned__owned_items__iterator_ref(x.clone());
        iter__vec_borrowed__owned_items__iterator_ref(&x.clone());
    }
    fn vec__borrowed_items() {
        let x: Vec<&i32> = vec![&1i32, &2, &3];
        into_iter__vec_owned__borrowed_items__iterator_ref(x.clone());
        into_iter__vec_borrowed__borrowed_items__iterator_ref_ref(&x.clone());
        iter__vec_owned__borrowed_items__iterator_ref_ref(x.clone());
        iter__vec_borrowed__borrowed_items__iterator_ref_ref(&x.clone());
    }
    fn into_iter__vec_owned__owned_items__iterator_owned<T>(x: Vec<T>) {
        let iter: IntoIter<T> = x.into_iter(); // actually into
        let iterator = &iter as &dyn Iterator<Item = T>;
    }
    fn into_iter__vec_owned__borrowed_items__iterator_ref<T>(x: Vec<&T>) {
        let iter: IntoIter<&T> = x.into_iter(); // actually into
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn into_iter__vec_borrowed__owned_items__iterator_ref<T>(x: &Vec<T>) {
        let iter: Iter<T> = x.into_iter(); // not into
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn into_iter__vec_borrowed__borrowed_items__iterator_ref_ref<T>(x: &Vec<&T>) {
        let iter: Iter<&T> = x.into_iter(); // not into but often practically equivalent
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn iter__vec_owned__owned_items__iterator_ref<T>(x: Vec<T>) {
        let iter: Iter<T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn iter__vec_owned__borrowed_items__iterator_ref_ref<T>(x: Vec<&T>) {
        let iter: Iter<&T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
    fn iter__vec_borrowed__owned_items__iterator_ref<T>(x: &Vec<T>) {
        let iter: Iter<T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &T>;
    }
    fn iter__vec_borrowed__borrowed_items__iterator_ref_ref<T>(x: &Vec<&T>) {
        let iter: Iter<&T> = x.iter();
        let iterator = &iter as &dyn Iterator<Item = &&T>;
    }
}
