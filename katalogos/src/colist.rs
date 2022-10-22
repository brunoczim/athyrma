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

pub struct Conil<A = ()>(pub Void, pub PhantomData<A>);

impl<A> Colist for Conil<A> {
    type Meta = A;
}

impl<A> fmt::Debug for Conil<A> {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

impl<A> Clone for Conil<A> {
    fn clone(&self) -> Self {
        match self.0 {}
    }
}

impl<A> Copy for Conil<A> {}

impl<A> PartialEq for Conil<A> {
    fn eq(&self, _other: &Self) -> bool {
        match self.0 {}
    }
}

impl<A> Eq for Conil<A> {}

impl<A> PartialOrd for Conil<A> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        match self.0 {}
    }
}

impl<A> Ord for Conil<A> {
    fn cmp(&self, _other: &Self) -> Ordering {
        match self.0 {}
    }
}

impl<A> Hash for Conil<A> {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
        match self.0 {}
    }
}

impl<A> fmt::Display for Conil<A> {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {}
    }
}

impl<A> Error for Conil<A> {}

impl<A, B> AsRef<B> for Conil<A>
where
    B: ?Sized,
{
    fn as_ref(&self) -> &B {
        match self.0 {}
    }
}

impl<A, B> AsMut<B> for Conil<A>
where
    B: ?Sized,
{
    fn as_mut(&mut self) -> &mut B {
        match self.0 {}
    }
}

impl<A> Future for Conil<A> {
    type Output = Conil<A>;

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

impl<H, T, A> AsRef<A> for Cocons<H, T>
where
    A: ?Sized,
    H: AsRef<A>,
    T: AsRef<A>,
{
    fn as_ref(&self) -> &A {
        match self {
            Cocons::Head(head) => head.as_ref(),
            Cocons::Tail(tail) => tail.as_ref(),
        }
    }
}

impl<H, T, A> AsMut<A> for Cocons<H, T>
where
    A: ?Sized,
    H: AsMut<A>,
    T: AsMut<A>,
{
    fn as_mut(&mut self) -> &mut A {
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
