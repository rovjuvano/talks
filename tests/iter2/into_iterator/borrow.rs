use rust::iter2::*;
#[test]
fn owned_items() {
    accepts_borrow!(IntoIterator<Borrow<T>>, [T; N]);
    accepts_with!(IntoIterator<Borrow<T>>, [&T; N], |x| borrowed_borrowed_to_owned(&x));
    accepts!(IntoIterator<Borrow<T>>, &[T; N]);
    accepts_with!(IntoIterator<Borrow<T>>, &[&T; N], |x| borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<Borrow<T>>, &[T]);
    accepts_with!(IntoIterator<Borrow<T>>, &[&T], |x| borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<Borrow<T>>, Vec<T>);
    accepts!(IntoIterator<Borrow<T>>, Vec<&T>);
    accepts!(IntoIterator<Borrow<T>>, &Vec<T>);
    accepts_with!(IntoIterator<Borrow<T>>, &Vec<&T>, |x| borrowed_borrowed_to_owned(x));

    accepts!(IntoIterator<Borrow<T>>, Iterator<T>);
    accepts_with!(IntoIterator<Borrow<T>>, Iterator<&T>, |x| borrowed_to_owned(x));
    accepts!(IntoIterator<Borrow<T>>, &mut Iterator<T>);
    accepts_with!(IntoIterator<Borrow<T>>, &mut Iterator<&T>, |x| borrowed_to_owned(x));

    accepts!(IntoIterator<Borrow<T>>, IntoIterator<T>);
    accepts_with!(IntoIterator<Borrow<T>>, IntoIterator<&T>, |x| borrowed_to_owned(x));
}
#[test]
fn borrowed_items() {
    accepts_borrow!(IntoIterator<Borrow<&T>>, [T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts_borrow!(IntoIterator<Borrow<&T>>, [&T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts!(IntoIterator<Borrow<&T>>, &[T; N]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts!(IntoIterator<Borrow<&T>>, &[&T; N]); // OR |x| x.iter() OR |x| x.into_iter()

    accepts!(IntoIterator<Borrow<&T>>, &[T]); // OR |x| x.iter() OR |x| x.into_iter()
    accepts!(IntoIterator<Borrow<&T>>, &[&T]); // OR |x| x.iter() OR |x| x.into_iter()

    accepts_borrow!(IntoIterator<Borrow<&T>>, Vec<T>); // OR |x| x.iter()
    accepts!(IntoIterator<Borrow<&T>>, Vec<&T>); // OR |x| x.iter() OR |x| x.into_iter()
    accepts!(IntoIterator<Borrow<&T>>, &Vec<T>); // OR |x| x.iter() OR |x| x.into_iter()
    accepts!(IntoIterator<Borrow<&T>>, &Vec<&T>); // OR |x| x.iter() OR |x| x.into_iter()

    accepts_with!(IntoIterator<Borrow<&T>>, Iterator<T>, |x| &collect_to_vec(x)); // not |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<Borrow<&T>>, Iterator<&T>);
    accepts_with!(IntoIterator<Borrow<&T>>, &mut Iterator<T>, |x| &collect_to_vec(x)); // not |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<Borrow<&T>>, &mut Iterator<&T>);

    accepts_with!(IntoIterator<Borrow<&T>>, IntoIterator<T>, |x| &collect_to_vec(x)); // not |x| collect_to_vec(x).iter()
    accepts!(IntoIterator<Borrow<&T>>, IntoIterator<&T>);
}
