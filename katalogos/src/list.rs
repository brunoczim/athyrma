use crate::colist::{Cocons, Conil};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter,
    marker::PhantomData,
};

pub trait List {
    type Meta: ?Sized;
}

pub struct Nil<M = ()>(PhantomData<M>)
where
    M: ?Sized;

impl<M> Default for Nil<M>
where
    M: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<M> Nil<M>
where
    M: ?Sized,
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<M> List for Nil<M>
where
    M: ?Sized,
{
    type Meta = M;
}

impl<M> fmt::Debug for Nil<M>
where
    M: ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_tuple("Nil").field(&self.0).finish()
    }
}

impl<M> Clone for Nil<M>
where
    M: ?Sized,
{
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<M> Copy for Nil<M> where M: ?Sized {}

impl<M> PartialEq for Nil<M>
where
    M: ?Sized,
{
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<M> Eq for Nil<M> where M: ?Sized {}

impl<M> PartialOrd for Nil<M>
where
    M: ?Sized,
{
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<M> Ord for Nil<M>
where
    M: ?Sized,
{
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<M> Hash for Nil<M>
where
    M: ?Sized,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state)
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
}

impl<M> IntoIterator for Nil<M>
where
    M: ?Sized,
{
    type Item = Conil<M>;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this, M> IntoIterator for &'this Nil<M>
where
    M: ?Sized,
{
    type Item = Conil<M>;
    type IntoIter = iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'this, M> IntoIterator for &'this mut Nil<M>
where
    M: ?Sized,
{
    type Item = Conil<M>;
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
