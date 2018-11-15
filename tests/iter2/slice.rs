use rust::iter2::*;
#[test]
fn owned_items() {
    accepts_borrow!(&[T], [T; N]);
    accepts_with!(&[T], [&T; N], |x| &borrowed_borrowed_to_owned(&x));
    accepts!(&[T], &[T; N]);
    accepts_with!(&[T], &[&T; N], |x| &borrowed_borrowed_to_owned(x));

    accepts_with!(&[T], &[T], |x| x);
    accepts_with!(&[T], &[&T], |x| &borrowed_borrowed_to_owned(x));

    accepts_borrow!(&[T], Vec<T>);
    accepts_with!(&[T], Vec<&T>, |x| &borrowed_to_owned(x));
    accepts_with!(&[T], &Vec<T>, |x| x);
    accepts_with!(&[T], &Vec<&T>, |x| &borrowed_borrowed_to_owned(x));

    accepts_with!(&[T], Iterator<T>, |x| &collect_to_vec(x));
    accepts_with!(&[T], Iterator<&T>, |x| &borrowed_to_owned(x));
    accepts_with!(&[T], &mut Iterator<T>, |x| &collect_to_vec(x));
    accepts_with!(&[T], &mut Iterator<&T>, |x| &borrowed_to_owned(x));

    accepts_with!(&[T], IntoIterator<T>, |x| &collect_to_vec(x));
    accepts_with!(&[T], IntoIterator<&T>, |x| &borrowed_to_owned(x));
}
#[test]
fn borrowed_items() {
    accepts_with!(&[&T], [T; N], |x| &owned_to_borrowed(&x));
    accepts_borrow!(&[&T], [&T; N]);
    accepts_with!(&[&T], &[T; N], |x| &owned_to_borrowed(x));
    accepts!(&[&T], &[&T; N]);

    accepts_with!(&[&T], &[T], |x| &owned_to_borrowed(x));
    accepts_with!(&[&T], &[&T], |x| x);

    accepts_with!(&[&T], Vec<T>, |x| &owned_to_borrowed(&x));
    accepts_borrow!(&[&T], Vec<&T>);
    accepts_with!(&[&T], &Vec<T>, |x| &owned_to_borrowed(x));
    accepts_with!(&[&T], &Vec<&T>, |x| x);

    accepts_with!(&[&T], Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
    accepts_with!(&[&T], Iterator<&T>, |x| &collect_to_vec(x));
    accepts_with!(&[&T], &mut Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
    accepts_with!(&[&T], &mut Iterator<&T>, |x| &collect_to_vec(x));

    accepts_with!(&[&T], IntoIterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
    accepts_with!(&[&T], IntoIterator<&T>, |x| &collect_to_vec(x));
}
