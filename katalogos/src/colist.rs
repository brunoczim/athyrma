use std::{
    cmp::Ordering,
    error::Error,
    fmt,
    future::Future,
    hash::{Hash, Hasher},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub trait Colist {
    type Meta;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Void {}

pub struct Conil<M = ()>(pub Void, pub PhantomData<M>);

impl<M> Colist for Conil<M> {
    type Meta = M;
}

impl<M> fmt::Debug for Conil<M> {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

impl<M> Clone for Conil<M> {
    fn clone(&self) -> Self {
        match self.0 {}
    }
}

impl<M> Copy for Conil<M> {}

impl<M> PartialEq for Conil<M> {
    fn eq(&self, _other: &Self) -> bool {
        match self.0 {}
    }
}

impl<M> Eq for Conil<M> {}

impl<M> PartialOrd for Conil<M> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        match self.0 {}
    }
}

impl<M> Ord for Conil<M> {
    fn cmp(&self, _other: &Self) -> Ordering {
        match self.0 {}
    }
}

impl<M> Hash for Conil<M> {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
        match self.0 {}
    }
}

impl<M> fmt::Display for Conil<M> {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

impl<M> Error for Conil<M> {}

impl<M, A> AsRef<A> for Conil<M>
where
    A: ?Sized,
{
    fn as_ref(&self) -> &A {
        match self.0 {}
    }
}

impl<M, A> AsMut<A> for Conil<M>
where
    A: ?Sized,
{
    fn as_mut(&mut self) -> &mut A {
        match self.0 {}
    }
}

impl<M> Future for Conil<M> {
    type Output = Conil<M>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(*self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cocons<H, T> {
    Head(H),
    Tail(T),
}

impl<H, T> Colist for Cocons<H, T>
where
    T: Colist,
{
    type Meta = T::Meta;
}

impl<H, T> fmt::Display for Cocons<H, T>
where
    H: fmt::Display,
    T: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cocons::Head(head) => head.fmt(fmtr),
            Cocons::Tail(tail) => tail.fmt(fmtr),
        }
    }
}

impl<H, T> Error for Cocons<H, T>
where
    H: Error,
    T: Error,
{
}

impl<H, T, M> AsRef<M> for Cocons<H, T>
where
    M: ?Sized,
    H: AsRef<M>,
    T: AsRef<M>,
{
    fn as_ref(&self) -> &M {
        match self {
            Cocons::Head(head) => head.as_ref(),
            Cocons::Tail(tail) => tail.as_ref(),
        }
    }
}

impl<H, T, M> AsMut<M> for Cocons<H, T>
where
    M: ?Sized,
    H: AsMut<M>,
    T: AsMut<M>,
{
    fn as_mut(&mut self) -> &mut M {
        match self {
            Cocons::Head(head) => head.as_mut(),
            Cocons::Tail(tail) => tail.as_mut(),
        }
    }
}

impl<H, T> Future for Cocons<H, T>
where
    H: Future,
    T: Future,
{
    type Output = Cocons<H::Output, T::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe {
            match self.get_unchecked_mut() {
                Cocons::Head(head) => {
                    Pin::new_unchecked(head).poll(cx).map(Cocons::Head)
                },
                Cocons::Tail(tail) => {
                    Pin::new_unchecked(tail).poll(cx).map(Cocons::Tail)
                },
            }
        }
    }
}
