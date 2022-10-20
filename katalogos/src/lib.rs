use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nil;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Conil {}

pub struct Cons<H, T> {
    pub head: H,
    pub tail: T,
}

pub enum Cocons<H, T> {
    Head(H),
    Tail(T),
}

pub trait ByRef<'this> {
    type Ref: 'this;

    fn by_ref(&'this self) -> Self::Ref;
}

pub trait ByMut<'this>: ByRef<'this> {
    type RefMut: 'this;

    fn by_mut(&'this mut self) -> Self::RefMut;
}

impl<'this> ByRef<'this> for Nil {
    type Ref = Nil;

    fn by_ref(&'this self) -> Self::Ref {
        *self
    }
}

impl<'this> ByRef<'this> for Conil {
    type Ref = Conil;

    fn by_ref(&'this self) -> Self::Ref {
        *self
    }
}

impl<'this, H, T> ByRef<'this> for Cons<H, T>
where
    H: 'this,
    T: ByRef<'this>,
{
    type Ref = Cons<&'this H, T::Ref>;

    fn by_ref(&'this self) -> Self::Ref {
        Cons { head: &self.head, tail: self.tail.by_ref() }
    }
}

impl<'this, H, T> ByRef<'this> for Cocons<H, T>
where
    H: 'this,
    T: ByRef<'this>,
{
    type Ref = Cocons<&'this H, T::Ref>;

    fn by_ref(&'this self) -> Self::Ref {
        match self {
            Cocons::Head(head) => Cocons::Head(head),
            Cocons::Tail(tail) => Cocons::Tail(tail.by_ref()),
        }
    }
}

impl<'this> ByMut<'this> for Nil {
    type RefMut = Nil;

    fn by_mut(&'this mut self) -> Self::RefMut {
        *self
    }
}

impl<'this> ByMut<'this> for Conil {
    type RefMut = Conil;

    fn by_mut(&'this mut self) -> Self::RefMut {
        *self
    }
}

impl<'this, H, T> ByMut<'this> for Cons<H, T>
where
    H: 'this,
    T: ByMut<'this>,
{
    type RefMut = Cons<&'this mut H, T::RefMut>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        Cons { head: &mut self.head, tail: self.tail.by_mut() }
    }
}

impl<'this, H, T> ByMut<'this> for Cocons<H, T>
where
    H: 'this,
    T: ByMut<'this>,
{
    type RefMut = Cocons<&'this mut H, T::RefMut>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        match self {
            Cocons::Head(head) => Cocons::Head(head),
            Cocons::Tail(tail) => Cocons::Tail(tail.by_mut()),
        }
    }
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
