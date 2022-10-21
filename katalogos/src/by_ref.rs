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
