use iter2::X;
#[derive(Debug, Default)]
pub struct XIntoIteratorOwned(Vec<X>);
impl XIntoIteratorOwned {
    pub fn new() -> Self {
        XIntoIteratorOwned(X::vec_owned())
    }
}
impl IntoIterator for XIntoIteratorOwned {
    type Item = X;
    type IntoIter = ::std::vec::IntoIter<X>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
#[derive(Debug, Default)]
pub struct XIntoIteratorBorrowed<'a>(Vec<&'a X>);
impl<'a> XIntoIteratorBorrowed<'a> {
    pub fn new() -> Self {
        XIntoIteratorBorrowed(X::vec_borrowed())
    }
}
impl<'a> IntoIterator for XIntoIteratorBorrowed<'a> {
    type Item = &'a X;
    type IntoIter = ::std::vec::IntoIter<&'a X>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
