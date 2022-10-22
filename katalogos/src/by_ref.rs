use crate::{
    colist::{Cocons, Conil},
    list::{Cons, Nil},
};

pub trait ByRef<'this> {
    type Ref: 'this;

    fn by_ref(&'this self) -> Self::Ref;
}

pub trait ByMut<'this>: ByRef<'this> {
    type RefMut: 'this;

    fn by_mut(&'this mut self) -> Self::RefMut;
}

impl<'this, M> ByRef<'this> for Nil<M>
where
    M: 'this,
{
    type Ref = Nil<&'this M>;

    fn by_ref(&'this self) -> Self::Ref {
        Nil::new()
    }
}

impl<'this, M> ByRef<'this> for Conil<M>
where
    M: 'this,
{
    type Ref = Conil<&'this M>;

    fn by_ref(&'this self) -> Self::Ref {
        self.coerce()
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

impl<'this, M> ByMut<'this> for Nil<M>
where
    M: 'this,
{
    type RefMut = Nil<&'this mut M>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        Nil::new()
    }
}

impl<'this, M> ByMut<'this> for Conil<M>
where
    M: 'this,
{
    type RefMut = Conil<&'this mut M>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        self.coerce()
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
