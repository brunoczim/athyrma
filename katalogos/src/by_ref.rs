use crate::coproduct::{Cocons, Conil};

pub trait ByRef<'this> {
    type Ref: 'this;

    fn by_ref(&'this self) -> Self::Ref;
}

pub trait ByMut<'this>: ByRef<'this> {
    type RefMut: 'this;

    fn by_mut(&'this mut self) -> Self::RefMut;
}

impl<'this, M> ByRef<'this> for Conil<M>
where
    M: 'this + ?Sized,
{
    type Ref = Conil<M>;

    fn by_ref(&'this self) -> Self::Ref {
        self.coerce()
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

impl<'this, M> ByMut<'this> for Conil<M>
where
    M: 'this + ?Sized,
{
    type RefMut = Conil<M>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        self.coerce()
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
