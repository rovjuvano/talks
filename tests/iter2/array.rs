mod owned {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts!([T; N], [T; N]);
        accepts_with!([T; N], [&T; N], |x| slice_to_array_3(&borrowed_borrowed_to_owned(&x)));
        accepts_with!([T; N], &[T; N], |x| slice_to_array_3(x));
        accepts_with!([T; N], &[&T; N], |x| slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!([T; N], &[T], |x| slice_to_array_3(x));
        accepts_with!([T; N], &[&T], |x| slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!([T; N], Vec<T>, |x| slice_to_array_3(&x));
        accepts_with!([T; N], Vec<&T>, |x| slice_to_array_3(&borrowed_to_owned(x)));
        accepts_with!([T; N], &Vec<T>, |x| slice_to_array_3(x));
        accepts_with!([T; N], &Vec<&T>, |x| slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!([T; N], Iterator<T>, |x| slice_to_array_3(&collect_to_vec(x)));
        accepts_with!([T; N], Iterator<&T>, |x| slice_to_array_3(&borrowed_to_owned(x)));
        accepts_with!([T; N], &mut Iterator<T>, |x| slice_to_array_3(&collect_to_vec(x)));
        accepts_with!([T; N], &mut Iterator<&T>, |x| slice_to_array_3(&borrowed_to_owned(x)));

        accepts_with!([T; N], IntoIterator<T>, |x| slice_to_array_3(&collect_to_vec(x)));
        accepts_with!([T; N], IntoIterator<&T>, |x| slice_to_array_3(&borrowed_to_owned(x)));
    }
    #[test]
    fn borrowed_items() {
        accepts_with!([&T; N], [T; N], |x| slice_to_array_3(&owned_to_borrowed(&x)));
        accepts!([&T; N], [&T; N]);
        accepts_with!([&T; N], &[T; N], |x| slice_to_array_3(&owned_to_borrowed(x)));
        accepts_with!([&T; N], &[&T; N], |x| x.clone());

        accepts_with!([&T; N], &[T], |x| slice_to_array_3(&owned_to_borrowed(x)));
        accepts_with!([&T; N], &[&T], |x| slice_to_array_3(x));

        accepts_with!([&T; N], Vec<T>, |x| slice_to_array_3(&owned_to_borrowed(&x)));
        accepts_with!([&T; N], Vec<&T>, |x| slice_to_array_3(&x));
        accepts_with!([&T; N], &Vec<T>, |x| slice_to_array_3(&owned_to_borrowed(x)));
        accepts_with!([&T; N], &Vec<&T>, |x| slice_to_array_3(x));

        accepts_with!([&T; N], Iterator<T>, |x| slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!([&T; N], Iterator<&T>, |x| slice_to_array_3(&collect_to_vec(x)));
        accepts_with!([&T; N], &mut Iterator<T>, |x| slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!([&T; N], &mut Iterator<&T>, |x| slice_to_array_3(&collect_to_vec(x)));

        accepts_with!([&T; N], IntoIterator<T>, |x| slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!([&T; N], IntoIterator<&T>, |x| slice_to_array_3(&collect_to_vec(x)));
    }
}
mod borrowed {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts_borrow!(&[T; N], [T; N]);
        accepts_with!(&[T; N], [&T; N], |x| &slice_to_array_3(&borrowed_borrowed_to_owned(&x)));
        accepts!(&[T; N], &[T; N]);
        accepts_with!(&[T; N], &[&T; N], |x| &slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!(&[T; N], &[T], |x| &slice_to_array_3(x));
        accepts_with!(&[T; N], &[&T], |x| &slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!(&[T; N], Vec<T>, |x| &slice_to_array_3(&x));
        accepts_with!(&[T; N], Vec<&T>, |x| &slice_to_array_3(&borrowed_to_owned(x)));
        accepts_with!(&[T; N], &Vec<T>, |x| &slice_to_array_3(x));
        accepts_with!(&[T; N], &Vec<&T>, |x| &slice_to_array_3(&borrowed_borrowed_to_owned(x)));

        accepts_with!(&[T; N], Iterator<T>, |x| &slice_to_array_3(&collect_to_vec(x)));
        accepts_with!(&[T; N], Iterator<&T>, |x| &slice_to_array_3(&borrowed_to_owned(x)));
        accepts_with!(&[T; N], &mut Iterator<T>, |x| &slice_to_array_3(&collect_to_vec(x)));
        accepts_with!(&[T; N], &mut Iterator<&T>, |x| &slice_to_array_3(&borrowed_to_owned(x)));

        accepts_with!(&[T; N], IntoIterator<T>, |x| &slice_to_array_3(&collect_to_vec(x)));
        accepts_with!(&[T; N], IntoIterator<&T>, |x| &slice_to_array_3(&borrowed_to_owned(x)));
    }
    #[test]
    fn borrowed_items() {
        accepts_with!(&[&T; N], [T; N], |x| &slice_to_array_3(&owned_to_borrowed(&x)));
        accepts_borrow!(&[&T; N], [&T; N]);
        accepts_with!(&[&T; N], &[T; N], |x| &slice_to_array_3(&owned_to_borrowed(x)));
        accepts!(&[&T; N], &[&T; N]);

        accepts_with!(&[&T; N], &[T], |x| &slice_to_array_3(&owned_to_borrowed(x)));
        accepts_with!(&[&T; N], &[&T], |x| &slice_to_array_3(x));

        accepts_with!(&[&T; N], Vec<T>, |x| &slice_to_array_3(&owned_to_borrowed(&x)));
        accepts_with!(&[&T; N], Vec<&T>, |x| &slice_to_array_3(&x));
        accepts_with!(&[&T; N], &Vec<T>, |x| &slice_to_array_3(&owned_to_borrowed(x)));
        accepts_with!(&[&T; N], &Vec<&T>, |x| &slice_to_array_3(x));

        accepts_with!(&[&T; N], Iterator<T>, |x| &slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!(&[&T; N], Iterator<&T>, |x| &slice_to_array_3(&collect_to_vec(x)));
        accepts_with!(&[&T; N], &mut Iterator<T>, |x| &slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!(&[&T; N], &mut Iterator<&T>, |x| &slice_to_array_3(&collect_to_vec(x)));

        accepts_with!(&[&T; N], IntoIterator<T>, |x| &slice_to_array_3(&owned_to_borrowed(&collect_to_vec(x))));
        accepts_with!(&[&T; N], IntoIterator<&T>, |x| &slice_to_array_3(&collect_to_vec(x)));
    }
}
