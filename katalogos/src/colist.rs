use std::{
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Conil {}

impl fmt::Display for Conil {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl Error for Conil {}

impl<A> AsRef<A> for Conil
where
    A: ?Sized,
{
    fn as_ref(&self) -> &A {
        match *self {}
    }
}

impl<A> AsMut<A> for Conil
where
    A: ?Sized,
{
    fn as_mut(&mut self) -> &mut A {
        match *self {}
    }
}

impl Future for Conil {
    type Output = Conil;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(*self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cocons<H, T> {
    Head(H),
    Tail(T),
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
