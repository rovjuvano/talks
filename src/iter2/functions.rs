#![allow(non_snake_case)]
pub use self::array::*;
mod array {
    use iter2::X;
    pub fn is_array_owned__items_owned(x: [X; 3]) {
        assert_eq!(x, new!([T; N]));
    }
    pub fn is_array_owned__items_borrowed(x: [&X; 3]) {
        assert_eq!(x, new!([&T; N]));
    }
    pub fn is_array_borrowed__items_owned(x: &[X; 3]) {
        assert_eq!(x, new!(&[T; N]));
    }
    pub fn is_array_borrowed__items_borrowed(x: &[&X; 3]) {
        assert_eq!(x, new!(&[&T; N]));
    }
}
pub use self::slice::*;
mod slice {
    use iter2::X;
    pub fn is_slice__items_owned(x: &[X]) {
        assert_eq!(x, new!(&[T]));
    }
    pub fn is_slice__items_borrowed(x: &[&X]) {
        assert_eq!(x, new!(&[&T]));
    }
}
pub use self::vec::*;
mod vec {
    use iter2::X;
    pub fn is_vec_owned__items_owned(x: Vec<X>) {
        assert_eq!(x, new!(Vec<T>));
    }
    pub fn is_vec_owned__items_borrowed(x: Vec<&X>) {
        assert_eq!(x, new!(Vec<&T>));
    }
    pub fn is_vec_borrowed__items_owned(x: &Vec<X>) {
        assert_eq!(x, new!(&Vec<T>));
    }
    pub fn is_vec_borrowed__items_borrowed(x: &Vec<&X>) {
        assert_eq!(x, new!(&Vec<&T>));
    }
}
pub use self::iterator::*;
mod iterator {
    use iter2::X;
    pub fn is_iterator_owned__items_owned(x: impl Iterator<Item = X>) {
        assert_eq!(x.collect::<Vec<X>>(), new!(Vec<T>));
    }
    pub fn is_iterator_owned__items_borrowed<'a>(x: impl Iterator<Item = &'a X>) {
        assert_eq!(x.collect::<Vec<&X>>(), new!(Vec<&T>));
    }
    pub fn is_iterator_borrowed__items_owned<'a>(x: &'a mut dyn Iterator<Item = X>) {
        assert_eq!(x.collect::<Vec<X>>(), new!(Vec<T>));
    }
    pub fn is_iterator_borrowed__items_borrowed<'a, 'b: 'a>(x: &'a mut dyn Iterator<Item = &'b X>) {
        assert_eq!(x.collect::<Vec<&X>>(), new!(Vec<&T>));
    }
}
pub use self::into_iterator::*;
mod into_iterator {
    use iter2::X;
    pub fn is_into_iterator__items_owned(x: impl IntoIterator<Item = X>) {
        assert_eq!(x.into_iter().collect::<Vec<X>>(), new!(Vec<T>));
    }
    pub fn is_into_iterator__items_borrowed<'a>(x: impl IntoIterator<Item = &'a X>) {
        assert_eq!(x.into_iter().collect::<Vec<&X>>(), new!(Vec<&T>));
    }
    pub use self::borrow::*;
    mod borrow {
        use iter2::X;
        pub fn is_into_iterator__borrow_items_owned<I>(x: I)
        where
            I: IntoIterator,
            I::Item: ::std::borrow::Borrow<X>,
        {
            use std::borrow::Borrow;
            assert_eq!(x.into_iter().map(|x| x.borrow().clone()).collect::<Vec<X>>(), new!(Vec<T>));
        }
        pub fn is_into_iterator__borrow_items_borrowed<'a, I>(x: I)
        where
            I: IntoIterator,
            I::Item: ::std::borrow::Borrow<&'a X>,
        {
            use std::borrow::Borrow;
            assert_eq!(x.into_iter().map(|x| *x.borrow()).collect::<Vec<&X>>(), new!(Vec<&T>));
        }
    }
    pub use self::deref::*;
    mod deref {
        use iter2::X;
        pub fn is_into_iterator__deref_items_owned<I>(x: I)
        where
            I: IntoIterator,
            I::Item: ::std::ops::Deref<Target = X>,
        {
            assert_eq!(x.into_iter().map(|x| (*x).clone()).collect::<Vec<X>>(), new!(Vec<T>));
            // OR
            // use std::ops::Deref;
            // assert_eq!(x.into_iter().map(|x| x.deref().clone()).collect::<Vec<X>>(), new!(Vec<T>));
        }
        pub fn is_into_iterator__deref_items_borrowed<'a, I>(x: I)
        where
            I: IntoIterator,
            I::Item: ::std::ops::Deref<Target = &'a X>,
        {
            assert_eq!(x.into_iter().map(|x| *x).collect::<Vec<&X>>(), new!(Vec<&T>));
            // OR
            // use std::ops::Deref;
            // assert_eq!(x.into_iter().map(|x| *x.deref()).collect::<Vec<&X>>(), new!(Vec<&T>));
        }
    }
    pub use self::traitx::*;
    mod traitx {
        use iter2::X;
        pub trait TraitAll: Sized {
            fn to_x(self) -> X;
        }
        impl TraitAll for X {
            fn to_x(self) -> X {
                self
            }
        }
        impl<'a> TraitAll for &'a X {
            fn to_x(self) -> X {
                self.clone()
            }
        }
        impl<'a, 'b> TraitAll for &'b &'a X {
            fn to_x(self) -> X {
                (*self).clone()
            }
        }

        pub fn is_into_iterator__trait_all<I>(x: I)
        where
            I: IntoIterator,
            I::Item: TraitAll,
        {
            assert_eq!(x.into_iter().map(|x| x.to_x()).collect::<Vec<X>>(), new!(Vec<T>));
        }
        pub trait TraitOwned: TraitAll {
            fn owned(self) -> X {
                self.to_x()
            }
        }
        impl TraitOwned for X {}
        pub fn is_into_iterator__trait_owned<I>(x: I)
        where
            I: IntoIterator,
            I::Item: TraitOwned,
        {
            assert_eq!(x.into_iter().map(|x| x.owned()).collect::<Vec<X>>(), new!(Vec<T>));
        }

        pub trait TraitBorrowed: TraitAll {
            fn borrowed(self) -> X {
                self.to_x()
            }
        }
        impl<'a> TraitBorrowed for &'a X {}
        pub fn is_into_iterator__trait_borrowed<I>(x: I)
        where
            I: IntoIterator,
            I::Item: TraitBorrowed,
        {
            assert_eq!(x.into_iter().map(|x| x.borrowed()).collect::<Vec<X>>(), new!(Vec<T>));
        }

        pub trait TraitBorrowedBorrowed: TraitAll {
            fn borrowed_borrowed(self) -> X {
                self.to_x()
            }
        }
        impl<'a, 'b> TraitBorrowedBorrowed for &'b &'a X {}
        pub fn is_into_iterator__trait_borrowed_borrowed<I>(x: I)
        where
            I: IntoIterator,
            I::Item: TraitBorrowedBorrowed,
        {
            assert_eq!(x.into_iter().map(|x| x.borrowed_borrowed()).collect::<Vec<X>>(), new!(Vec<T>));
        }
    }
}
