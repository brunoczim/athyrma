use crate::colist::{Cocons, Conil};
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nil;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cons<H, T> {
    pub head: H,
    pub tail: T,
}

impl IntoIterator for Nil {
    type Item = Conil;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this> IntoIterator for &'this Nil {
    type Item = Conil;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this> IntoIterator for &'this mut Nil {
    type Item = Conil;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<H, T> IntoIterator for Cons<H, T>
where
    T: IntoIterator,
{
    type Item = Cocons<H, T::Item>;
    type IntoIter = iter::Chain<
        iter::Once<Self::Item>,
        iter::Map<T::IntoIter, fn(T::Item) -> Self::Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let head = iter::once(Cocons::Head(self.head));
        let tail = self.tail.into_iter().map(Cocons::Tail as _);
        head.chain(tail)
    }
}

impl<'this, H, T> IntoIterator for &'this Cons<H, T>
where
    &'this T: IntoIterator,
{
    type Item = Cocons<&'this H, <&'this T as IntoIterator>::Item>;
    type IntoIter = iter::Chain<
        iter::Once<Self::Item>,
        iter::Map<
            <&'this T as IntoIterator>::IntoIter,
            fn(<&'this T as IntoIterator>::Item) -> Self::Item,
        >,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let head = iter::once(Cocons::Head(&self.head));
        let tail = self.tail.into_iter().map(Cocons::Tail as _);
        head.chain(tail)
    }
}

impl<'this, H, T> IntoIterator for &'this mut Cons<H, T>
where
    &'this mut T: IntoIterator,
{
    type Item = Cocons<&'this mut H, <&'this mut T as IntoIterator>::Item>;
    type IntoIter = iter::Chain<
        iter::Once<Self::Item>,
        iter::Map<
            <&'this mut T as IntoIterator>::IntoIter,
            fn(<&'this mut T as IntoIterator>::Item) -> Self::Item,
        >,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let head = iter::once(Cocons::Head(&mut self.head));
        let tail = self.tail.into_iter().map(Cocons::Tail as _);
        head.chain(tail)
    }
}
