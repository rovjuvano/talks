use rust::iter2::*;
#[test]
fn owned_items() {
    accepts_with!(IntoIterator<T>, [T; N], |x| borrowed_to_owned(&x));
    accepts_with!(IntoIterator<T>, [&T; N], |x| borrowed_borrowed_to_owned(&x));
    accepts_with!(IntoIterator<T>, &[T; N], |x| borrowed_to_owned(x));
    accepts_with!(IntoIterator<T>, &[&T; N], |x| borrowed_borrowed_to_owned(x));

    accepts_with!(IntoIterator<T>, &[T], |x| borrowed_to_owned(x));
    accepts_with!(IntoIterator<T>, &[&T], |x| borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<T>, Vec<T>);
    accepts_with!(IntoIterator<T>, Vec<&T>, |x| borrowed_to_owned(x));
    accepts_with!(IntoIterator<T>, &Vec<T>, |x| borrowed_to_owned(x));
    accepts_with!(IntoIterator<T>, &Vec<&T>, |x| borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<T>, Iterator<T>);
    accepts_with!(IntoIterator<T>, Iterator<&T>, |x| borrowed_to_owned(x));
    accepts!(IntoIterator<T>, &mut Iterator<T>);
    accepts_with!(IntoIterator<T>, &mut Iterator<&T>, |x| borrowed_to_owned(x));

    accepts!(IntoIterator<T>, IntoIterator<T>);
    accepts_with!(IntoIterator<T>, IntoIterator<&T>, |x| borrowed_to_owned(x));
}
#[test]
fn borrowed_items() {
    accepts_borrow!(IntoIterator<&T>, [T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<&T>, [&T; N], |x| &borrowed_borrowed_to_owned(&x));
    accepts!(IntoIterator<&T>, &[T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<&T>, &[&T; N], |x| &borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<&T>, &[T]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<&T>, &[&T], |x| &borrowed_borrowed_to_owned(x));

    accepts_borrow!(IntoIterator<&T>, Vec<T>); // OR |x| x.iter()
    accepts!(IntoIterator<&T>, Vec<&T>);
    accepts!(IntoIterator<&T>, &Vec<T>); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_with!(IntoIterator<&T>, &Vec<&T>, |x| &borrowed_borrowed_to_owned(x));

    accepts_with!(IntoIterator<&T>, Iterator<T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<&T>, Iterator<&T>);
    accepts_with!(IntoIterator<&T>, &mut Iterator<T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<&T>, &mut Iterator<&T>);

    accepts_with!(IntoIterator<&T>, IntoIterator<T>, |x| &collect_to_vec(x)); // OR |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<&T>, IntoIterator<&T>);
}
mod borrow;
mod deref;
mod traitx;
