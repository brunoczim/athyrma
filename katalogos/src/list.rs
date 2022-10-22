use crate::colist::{Cocons, Conil};
use std::iter;

pub trait List {
    type Meta;

    fn meta(&self) -> &Self::Meta;

    fn meta_mut(&mut self) -> &mut Self::Meta;

    fn into_meta(self) -> Self::Meta;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nil<A = ()>(pub A);

impl<A> List for Nil<A> {
    type Meta = A;

    fn meta(&self) -> &Self::Meta {
        &self.0
    }

    fn meta_mut(&mut self) -> &mut Self::Meta {
        &mut self.0
    }

    fn into_meta(self) -> Self::Meta {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cons<H, T> {
    pub head: H,
    pub tail: T,
}

impl<H, T> List for Cons<H, T>
where
    T: List,
{
    type Meta = T::Meta;

    fn meta(&self) -> &Self::Meta {
        self.tail.meta()
    }

    fn meta_mut(&mut self) -> &mut Self::Meta {
        self.tail.meta_mut()
    }

    fn into_meta(self) -> Self::Meta {
        self.tail.into_meta()
    }
}

impl<A> IntoIterator for Nil<A> {
    type Item = Conil<A>;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this, A> IntoIterator for &'this Nil<A> {
    type Item = Conil<A>;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this, A> IntoIterator for &'this mut Nil<A> {
    type Item = Conil<A>;
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
