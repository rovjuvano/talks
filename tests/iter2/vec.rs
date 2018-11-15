mod owned {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts_with!(Vec<T>, [T; N], |x| x.to_vec());
        accepts_with!(Vec<T>, [&T; N], |x| borrowed_borrowed_to_owned(&x));
        accepts_with!(Vec<T>, &[T; N], |x| x.to_vec());
        accepts_with!(Vec<T>, &[&T; N], |x| borrowed_borrowed_to_owned(x));

        accepts_with!(Vec<T>, &[T], |x| x.to_vec());
        accepts_with!(Vec<T>, &[&T], |x| borrowed_borrowed_to_owned(x));

        accepts!(Vec<T>, Vec<T>);
        accepts_with!(Vec<T>, Vec<&T>, |x| borrowed_to_owned(x));
        accepts_with!(Vec<T>, &Vec<T>, |x| x.clone());
        accepts_with!(Vec<T>, &Vec<&T>, |x| borrowed_borrowed_to_owned(x));

        accepts_with!(Vec<T>, Iterator<T>, |x| collect_to_vec(x));
        accepts_with!(Vec<T>, Iterator<&T>, |x| borrowed_to_owned(x));
        accepts_with!(Vec<T>, &mut Iterator<T>, |x| collect_to_vec(x));
        accepts_with!(Vec<T>, &mut Iterator<&T>, |x| borrowed_to_owned(x));

        accepts_with!(Vec<T>, IntoIterator<T>, |x| collect_to_vec(x));
        accepts_with!(Vec<T>, IntoIterator<&T>, |x| borrowed_to_owned(x));
    }
    #[test]
    fn borrowed_items() {
        accepts_with!(Vec<&T>, [T; N], |x| owned_to_borrowed(&x));
        accepts_with!(Vec<&T>, [&T; N], |x| x.to_vec());
        accepts_with!(Vec<&T>, &[T; N], |x| owned_to_borrowed(x));
        accepts_with!(Vec<&T>, &[&T; N], |x| x.to_vec());

        accepts_with!(Vec<&T>, &[T], |x| owned_to_borrowed(x));
        accepts_with!(Vec<&T>, &[&T], |x| x.to_vec());

        accepts_with!(Vec<&T>, Vec<T>, |x| owned_to_borrowed(&x));
        accepts!(Vec<&T>, Vec<&T>);
        accepts_with!(Vec<&T>, &Vec<T>, |x| owned_to_borrowed(x));
        accepts_with!(Vec<&T>, &Vec<&T>, |x| x.clone());

        accepts_with!(Vec<&T>, Iterator<T>, |x| owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(Vec<&T>, Iterator<&T>, |x| collect_to_vec(x));
        accepts_with!(Vec<&T>, &mut Iterator<T>, |x| owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(Vec<&T>, &mut Iterator<&T>, |x| collect_to_vec(x));

        accepts_with!(Vec<&T>, IntoIterator<T>, |x| owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(Vec<&T>, IntoIterator<&T>, |x| collect_to_vec(x));
    }
}
mod borrowed {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts_with!(&Vec<T>, [T; N], |x| &x.to_vec());
        accepts_with!(&Vec<T>, [&T; N], |x| &borrowed_borrowed_to_owned(&x));
        accepts_with!(&Vec<T>, &[T; N], |x| &x.to_vec());
        accepts_with!(&Vec<T>, &[&T; N], |x| &borrowed_borrowed_to_owned(x));

        accepts_with!(&Vec<T>, &[T], |x| &x.to_vec());
        accepts_with!(&Vec<T>, &[&T], |x| &borrowed_borrowed_to_owned(x));

        accepts_borrow!(&Vec<T>, Vec<T>);
        accepts_with!(&Vec<T>, Vec<&T>, |x| &borrowed_to_owned(x));
        accepts!(&Vec<T>, &Vec<T>);
        accepts_with!(&Vec<T>, &Vec<&T>, |x| &borrowed_borrowed_to_owned(x));

        accepts_with!(&Vec<T>, Iterator<T>, |x| &collect_to_vec(x));
        accepts_with!(&Vec<T>, Iterator<&T>, |x| &borrowed_to_owned(x));
        accepts_with!(&Vec<T>, &mut Iterator<T>, |x| &collect_to_vec(x));
        accepts_with!(&Vec<T>, &mut Iterator<&T>, |x| &borrowed_to_owned(x));

        accepts_with!(&Vec<T>, IntoIterator<T>, |x| &collect_to_vec(x));
        accepts_with!(&Vec<T>, IntoIterator<&T>, |x| &borrowed_to_owned(x));
    }
    #[test]
    fn borrowed_items() {
        accepts_with!(&Vec<&T>, [T; N], |x| &owned_to_borrowed(&x));
        accepts_with!(&Vec<&T>, [&T; N], |x| &x.to_vec());
        accepts_with!(&Vec<&T>, &[T; N], |x| &owned_to_borrowed(x));
        accepts_with!(&Vec<&T>, &[&T; N], |x| &x.to_vec());

        accepts_with!(&Vec<&T>, &[T], |x| &owned_to_borrowed(x));
        accepts_with!(&Vec<&T>, &[&T], |x| &x.to_vec());

        accepts_with!(&Vec<&T>, Vec<T>, |x| &owned_to_borrowed(&x));
        accepts_borrow!(&Vec<&T>, Vec<&T>);
        accepts_with!(&Vec<&T>, &Vec<T>, |x| &owned_to_borrowed(x));
        accepts!(&Vec<&T>, &Vec<&T>);

        accepts_with!(&Vec<&T>, Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(&Vec<&T>, Iterator<&T>, |x| &collect_to_vec(x));
        accepts_with!(&Vec<&T>, &mut Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(&Vec<&T>, &mut Iterator<&T>, |x| &collect_to_vec(x));

        accepts_with!(&Vec<&T>, IntoIterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x)));
        accepts_with!(&Vec<&T>, IntoIterator<&T>, |x| &collect_to_vec(x));
    }
}
