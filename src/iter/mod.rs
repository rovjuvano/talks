//! Comparison of ways to accept a list of items as a function parameter
mod as_ref_into_iterator;
mod borrow_into_iterator_borrow;
mod into_iterator_as_ref_owned;
mod into_iterator_as_ref_ref;
mod into_iterator_borrow_borrow_owned;
mod into_iterator_borrow_borrow_ref;
mod into_iterator_borrow_deref_owned;
mod into_iterator_borrow_owned;
mod into_iterator_borrow_ref;
mod into_iterator_cow_owned;
mod into_iterator_cow_ref;
mod into_iterator_debug;
mod into_iterator_deref_borrow_owned;
mod into_iterator_deref_owned;
mod into_iterator_deref_ref;
mod into_iterator_into_owned;
mod into_iterator_into_ref;
mod into_iterator_owned;
mod into_iterator_ref;
mod into_iterator_trait;
mod into_ref_vec_owned;
mod into_ref_vec_ref;
mod into_slice_owned;
mod into_slice_ref;
mod into_vec_into_owned;
mod into_vec_into_ref;
mod into_vec_owned;
mod into_vec_ref;
mod iterator_into_owned;
mod iterator_into_ref;
mod iterator_owned;
mod iterator_ref;
mod ref_vec_into_owned;
mod ref_vec_into_ref;
mod ref_vec_owned;
mod ref_vec_ref;
mod slice_into_owned;
mod slice_into_ref;
mod slice_owned;
mod slice_ref;
mod vec_into_owned;
mod vec_into_ref;
mod vec_owned;
mod vec_ref;
fn print(i: usize, x: i32) {
    println!("  [{:?}]: {:?}", i, x);
}
pub fn main() {
    into_iterator_debug::main();

    vec_owned::main();
    vec_ref::main();
    ref_vec_owned::main();
    ref_vec_ref::main();
    slice_owned::main();
    slice_ref::main();

    vec_into_owned::main();
    vec_into_ref::main();
    ref_vec_into_owned::main();
    ref_vec_into_ref::main();
    slice_into_owned::main();
    slice_into_ref::main();

    into_vec_owned::main();
    into_vec_ref::main();
    into_ref_vec_owned::main();
    into_ref_vec_ref::main();
    into_slice_owned::main();
    into_slice_ref::main();

    into_vec_into_owned::main();
    into_vec_into_ref::main();
    // into_ref_vec_into_owned::main();
    // into_ref_vec_into_ref::main();
    // into_slice_into_owned::main();
    // into_slice_into_ref::main();

    iterator_owned::main();
    iterator_ref::main();
    iterator_into_owned::main();
    iterator_into_ref::main();

    into_iterator_owned::main(); // FAQ
    into_iterator_ref::main();
    into_iterator_into_owned::main();
    into_iterator_into_ref::main();

    into_iterator_as_ref_owned::main(); // FAQ
    into_iterator_cow_owned::main(); // FAQ
    into_iterator_deref_owned::main();
    into_iterator_borrow_owned::main(); // common question as_ref vs borrow

    into_iterator_as_ref_ref::main();
    into_iterator_cow_ref::main();
    into_iterator_deref_ref::main();
    into_iterator_borrow_ref::main();

    into_iterator_borrow_borrow_owned::main();
    into_iterator_borrow_borrow_ref::main();
    into_iterator_deref_borrow_owned::main();
    into_iterator_borrow_deref_owned::main();

    as_ref_into_iterator::main();
    borrow_into_iterator_borrow::main();

    into_iterator_trait::main();
}
