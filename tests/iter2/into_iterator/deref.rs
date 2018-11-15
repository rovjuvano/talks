use rust::iter2::*;
#[test]
fn owned_items() {
    accepts_borrow!(IntoIterator<Deref<T>>, [T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<Deref<T>>, [&T; N], |x| &borrowed_borrowed_to_owned(&x));
    accepts!(IntoIterator<Deref<T>>, &[T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<Deref<T>>, &[&T; N], |x| &borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<Deref<T>>, &[T]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<Deref<T>>, &[&T], |x| &borrowed_borrowed_to_owned(x));

    accepts_borrow!(IntoIterator<Deref<T>>, Vec<T>); // OR x.iter()
    accepts!(IntoIterator<Deref<T>>, Vec<&T>);
    accepts!(IntoIterator<Deref<T>>, &Vec<T>); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<Deref<T>>, &Vec<&T>, |x| &borrowed_borrowed_to_owned(x));

    accepts_with!(IntoIterator<Deref<T>>, Iterator<T>, |x| &collect_to_vec(x));
    accepts!(IntoIterator<Deref<T>>, Iterator<&T>);
    accepts_with!(IntoIterator<Deref<T>>, &mut Iterator<T>, |x| &collect_to_vec(x));
    accepts!(IntoIterator<Deref<T>>, &mut Iterator<&T>);

    accepts_with!(IntoIterator<Deref<T>>, IntoIterator<T>, |x| &collect_to_vec(x));
    accepts!(IntoIterator<Deref<T>>, IntoIterator<&T>);
}
#[test]
fn borrowed_items() {
    accepts_with!(IntoIterator<Deref<&T>>, [T; N], |x| &collect_to_vec(&x)); // OR .iter()
    accepts_borrow!(IntoIterator<Deref<&T>>, [&T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<Deref<&T>>, &[T; N], |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<Deref<&T>>, &[&T; N]); // OR |x| x.iter() OR |x| x.into_iter()

    accepts_with!(IntoIterator<Deref<&T>>, &[T], |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<Deref<&T>>, &[&T]); // OR |x| x.iter() OR |x| x.into_iter()

    accepts_with!(IntoIterator<Deref<&T>>, Vec<T>, |x| &owned_to_borrowed(&x)); // OR |x| &owned_to_borrowed(&x).iter()
    accepts_borrow!(IntoIterator<Deref<&T>>, Vec<&T>); // OR |x| x.iter()
    accepts_with!(IntoIterator<Deref<&T>>, &Vec<T>, |x| &owned_to_borrowed(x)); // OR |x| &owned_to_borrowed(x).iter()
    accepts!(IntoIterator<Deref<&T>>, &Vec<&T>); // OR |x| x.iter() OR |x| x.into_iter()

    accepts_with!(IntoIterator<Deref<&T>>, Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x))); // OR |x| owned_to_borrowed(&collect_to_vec(x)).iter()
    accepts_with!(IntoIterator<Deref<&T>>, Iterator<&T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts_with!(IntoIterator<Deref<&T>>, &mut Iterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x))); // OR |x| owned_to_borrowed(&collect_to_vec(x)).iter()
    accepts_with!(IntoIterator<Deref<&T>>, &mut Iterator<&T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()

    accepts_with!(IntoIterator<Deref<&T>>, IntoIterator<T>, |x| &owned_to_borrowed(&collect_to_vec(x))); // OR |x| owned_to_borrowed(&collect_to_vec(x)).iter()
    accepts_with!(IntoIterator<Deref<&T>>, IntoIterator<&T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
}
