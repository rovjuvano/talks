use rust::iter2::*;
#[test]
fn all() {
    accepts_borrow!(IntoIterator<TraitAll>, [T; N]);
    accepts_borrow!(IntoIterator<TraitAll>, [&T; N]);
    accepts!(IntoIterator<TraitAll>, &[T; N]);
    accepts!(IntoIterator<TraitAll>, &[&T; N]);

    accepts!(IntoIterator<TraitAll>, &[T]);
    accepts!(IntoIterator<TraitAll>, &[&T]);

    accepts!(IntoIterator<TraitAll>, Vec<T>);
    accepts!(IntoIterator<TraitAll>, Vec<&T>);
    accepts!(IntoIterator<TraitAll>, &Vec<T>);
    accepts!(IntoIterator<TraitAll>, &Vec<&T>);

    accepts!(IntoIterator<TraitAll>, Iterator<T>);
    accepts!(IntoIterator<TraitAll>, Iterator<&T>);
    accepts!(IntoIterator<TraitAll>, &mut Iterator<T>);
    accepts!(IntoIterator<TraitAll>, &mut Iterator<&T>);

    accepts!(IntoIterator<TraitAll>, IntoIterator<T>);
    accepts!(IntoIterator<TraitAll>, IntoIterator<&T>);
}
#[test]
fn each() {
    accepts_borrow!(IntoIterator<TraitBorrowed>, [T; N]);
    accepts_borrow!(IntoIterator<TraitBorrowedBorrowed>, [&T; N]);
    accepts!(IntoIterator<TraitBorrowed>, &[T; N]);
    accepts!(IntoIterator<TraitBorrowedBorrowed>, &[&T; N]);

    accepts!(IntoIterator<TraitBorrowed>, &[T]);
    accepts!(IntoIterator<TraitBorrowedBorrowed>, &[&T]);

    accepts!(IntoIterator<TraitOwned>, Vec<T>);
    accepts!(IntoIterator<TraitBorrowed>, Vec<&T>);
    accepts!(IntoIterator<TraitBorrowed>, &Vec<T>);
    accepts!(IntoIterator<TraitBorrowedBorrowed>, &Vec<&T>);

    accepts!(IntoIterator<TraitOwned>, Iterator<T>);
    accepts!(IntoIterator<TraitBorrowed>, Iterator<&T>);
    accepts!(IntoIterator<TraitOwned>, &mut Iterator<T>);
    accepts!(IntoIterator<TraitBorrowed>, &mut Iterator<&T>);

    accepts!(IntoIterator<TraitOwned>, IntoIterator<T>);
    accepts!(IntoIterator<TraitBorrowed>, IntoIterator<&T>);
}
