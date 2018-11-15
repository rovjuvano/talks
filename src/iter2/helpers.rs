pub fn borrowed_borrowed_to_owned<'a: 'b, 'b, I, T>(container: I) -> Vec<T>
where
    I: IntoIterator<Item = &'b &'a T>,
    T: Clone + 'a,
{
    container.into_iter().map(|x| (*x).clone()).collect()
}
pub fn borrowed_to_owned<'a, I, T>(container: I) -> Vec<T>
where
    I: IntoIterator<Item = &'a T>,
    T: Clone + 'a,
{
    container.into_iter().map(|x| (*x).clone()).collect()
}
pub fn collect_to_vec<I, T>(container: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    container.into_iter().collect::<Vec<_>>()
}
pub fn owned_to_borrowed<'a, I, T>(container: I) -> Vec<&'a T>
where
    I: IntoIterator<Item = &'a T>,
{
    container.into_iter().collect()
}
// for more options: https://stackoverflow.com/questions/25428920/how-to-get-a-slice-as-an-array-in-rust
pub fn slice_to_array_3<T: Clone>(slice: &[T]) -> [T; 3] {
    [slice[0].clone(), slice[1].clone(), slice[2].clone()]
    // let array = [<T as Default>::default(); 3];
    // array.copy_from_slice(slice);
    // array
}
