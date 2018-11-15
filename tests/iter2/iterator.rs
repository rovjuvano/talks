mod owned {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts_with!(Iterator<T>, [T; N], |x| x.to_vec().into_iter());
        accepts_with!(Iterator<T>, [&T; N], |x| borrowed_borrowed_to_owned(&x).into_iter());
        accepts_with!(Iterator<T>, &[T; N], |x| x.to_vec().into_iter());
        accepts_with!(Iterator<T>, &[&T; N], |x| borrowed_borrowed_to_owned(x).into_iter());

        accepts_with!(Iterator<T>, &[T], |x| x.to_vec().into_iter());
        accepts_with!(Iterator<T>, &[&T], |x| borrowed_borrowed_to_owned(x).into_iter());

        accepts_with!(Iterator<T>, Vec<T>, |x| x.into_iter());
        accepts_with!(Iterator<T>, Vec<&T>, |x| borrowed_to_owned(x).into_iter());
        accepts_with!(Iterator<T>, &Vec<T>, |x| x.clone().into_iter());
        accepts_with!(Iterator<T>, &Vec<&T>, |x| borrowed_borrowed_to_owned(x).into_iter());

        accepts!(Iterator<T>, Iterator<T>);
        accepts_with!(Iterator<T>, Iterator<&T>, |x| borrowed_to_owned(x).into_iter());
        accepts!(Iterator<T>, &mut Iterator<T>);
        accepts_with!(Iterator<T>, &mut Iterator<&T>, |x| borrowed_to_owned(x).into_iter());

        accepts_with!(Iterator<T>, IntoIterator<T>, |x| x.into_iter());
        accepts_with!(Iterator<T>, IntoIterator<&T>, |x| borrowed_to_owned(x).into_iter());
    }
    #[test]
    fn borrowed_items() {
        accepts_with!(Iterator<&T>, [T; N], |x| x.iter()); // OR |x| x.into_iter()
        accepts_with!(Iterator<&T>, [&T; N], |x| x.to_vec().into_iter());
        accepts_with!(Iterator<&T>, &[T; N], |x| x.iter()); // OR |x| x.into_iter()
        accepts_with!(Iterator<&T>, &[&T; N], |x| x.to_vec().into_iter());

        accepts_with!(Iterator<&T>, &[T], |x| x.iter()); // OR |x| x.into_iter()
        accepts_with!(Iterator<&T>, &[&T], |x| x.to_vec().into_iter());

        accepts_with!(Iterator<&T>, Vec<T>, |x| x.iter());
        accepts_with!(Iterator<&T>, Vec<&T>, |x| x.into_iter());
        accepts_with!(Iterator<&T>, &Vec<T>, |x| x.iter());
        accepts_with!(Iterator<&T>, &Vec<&T>, |x| x.clone().into_iter());

        accepts_with!(Iterator<&T>, Iterator<T>, |x| collect_to_vec(x).iter()); // not x.map(|x| &x)
        accepts!(Iterator<&T>, Iterator<&T>);
        accepts_with!(Iterator<&T>, &mut Iterator<T>, |x| collect_to_vec(x).iter());
        accepts!(Iterator<&T>, &mut Iterator<&T>);

        accepts_with!(Iterator<&T>, IntoIterator<T>, |x| collect_to_vec(x).iter()); // not x.map(|x| &x)
        accepts_with!(Iterator<&T>, IntoIterator<&T>, |x| x.into_iter());
    }
}
mod borrowed {
    use rust::iter2::*;
    #[test]
    fn owned_items() {
        accepts_with!(&mut Iterator<T>, [T; N], |x| &mut x.to_vec().into_iter());
        accepts_with!(&mut Iterator<T>, [&T; N], |x| &mut borrowed_borrowed_to_owned(&x).into_iter());
        accepts_with!(&mut Iterator<T>, &[T; N], |x| &mut x.to_vec().into_iter());
        accepts_with!(&mut Iterator<T>, &[&T; N], |x| &mut borrowed_borrowed_to_owned(x).into_iter());

        accepts_with!(&mut Iterator<T>, &[T], |x| &mut x.to_vec().into_iter());
        accepts_with!(&mut Iterator<T>, &[&T], |x| &mut borrowed_borrowed_to_owned(x).into_iter());

        accepts_with!(&mut Iterator<T>, Vec<T>, |x| &mut x.into_iter());
        accepts_with!(&mut Iterator<T>, Vec<&T>, |x| &mut borrowed_to_owned(x).into_iter());
        accepts_with!(&mut Iterator<T>, &Vec<T>, |x| &mut x.clone().into_iter());
        accepts_with!(&mut Iterator<T>, &Vec<&T>, |x| &mut borrowed_borrowed_to_owned(x).into_iter());

        accepts_with!(&mut Iterator<T>, Iterator<T>, |mut x| &mut x);
        accepts_with!(&mut Iterator<T>, Iterator<&T>, |x| &mut borrowed_to_owned(x).into_iter());
        accepts!(&mut Iterator<T>, &mut Iterator<T>);
        accepts_with!(&mut Iterator<T>, &mut Iterator<&T>, |x| &mut borrowed_to_owned(x).into_iter());

        accepts_with!(&mut Iterator<T>, IntoIterator<T>, |x| &mut x.into_iter());
        accepts_with!(&mut Iterator<T>, IntoIterator<&T>, |x| &mut borrowed_to_owned(x).into_iter());
    }
    #[test]
    fn borrowed_items() {
        accepts_with!(&mut Iterator<&T>, [T; N], |x| &mut x.iter()); // OR |x| &mut x.into_iter()
        accepts_with!(&mut Iterator<&T>, [&T; N], |x| &mut x.to_vec().into_iter());
        accepts_with!(&mut Iterator<&T>, &[T; N], |x| &mut x.iter()); // OR |x| &mut x.into_iter()
        accepts_with!(&mut Iterator<&T>, &[&T; N], |x| &mut x.to_vec().into_iter());

        accepts_with!(&mut Iterator<&T>, &[T], |x| &mut x.iter()); // OR |x| &mut x.into_iter()
        accepts_with!(&mut Iterator<&T>, &[T], |x| &mut x.into_iter()); // OR |x| &mut x.into_iter()
        accepts_with!(&mut Iterator<&T>, &[&T], |x| &mut x.to_vec().into_iter());

        accepts_with!(&mut Iterator<&T>, Vec<T>, |x| &mut x.iter());
        accepts_with!(&mut Iterator<&T>, Vec<&T>, |x| &mut x.into_iter());
        accepts_with!(&mut Iterator<&T>, &Vec<T>, |x| &mut x.iter());
        accepts_with!(&mut Iterator<&T>, &Vec<&T>, |x| &mut x.clone().into_iter());

        accepts_with!(&mut Iterator<&T>, Iterator<T>, |x| &mut collect_to_vec(x).iter()); // not |x| x.iter() OR |x| &mut x.map(|x| &x)
        accepts_with!(&mut Iterator<&T>, Iterator<&T>, |mut x| &mut x);
        accepts_with!(&mut Iterator<&T>, &mut Iterator<T>, |x| &mut collect_to_vec(x).iter());
        accepts!(&mut Iterator<&T>, &mut Iterator<&T>);

        accepts_with!(&mut Iterator<&T>, IntoIterator<T>, |x| &mut collect_to_vec(x).iter()); // not |x| x.iter() OR |x| &mut x.map(|x| &x)
        accepts_with!(&mut Iterator<&T>, IntoIterator<&T>, |x| &mut x.into_iter());
    }
}
