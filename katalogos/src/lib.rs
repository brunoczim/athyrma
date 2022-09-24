use std::{rc::Rc, sync::Arc};

pub trait List {}

impl<'this, L> List for &'this L where L: List + ?Sized {}

impl<'this, L> List for &'this mut L where L: List + ?Sized {}

impl<L> List for Box<L> where L: List + ?Sized {}

impl<L> List for Rc<L> where L: List + ?Sized {}

impl<L> List for Arc<L> where L: List + ?Sized {}

pub trait Cons<A>: List {
    type Output: List;

    fn cons(self, head: A) -> Self::Output;
}

pub trait Uncons: List {
    type Head;
    type Tail: List;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)>;
}

impl<T> List for [T] {}

impl<'this, T> Uncons for &'this [T] {
    type Head = &'this T;
    type Tail = Self;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        self.split_first()
    }
}

impl<T> List for Vec<T> {}

impl<T> Cons<T> for Vec<T> {
    type Output = Self;

    fn cons(mut self, head: T) -> Self::Output {
        self.insert(0, head);
        self
    }
}

impl<T> Uncons for Vec<T> {
    type Head = T;
    type Tail = Self;

    fn uncons(mut self) -> Option<(Self::Head, Self::Tail)> {
        if self.len() == 0 {
            None
        } else {
            let head = self.remove(0);
            Some((head, self))
        }
    }
}

impl<'elem, T> Uncons for &'elem Vec<T> {
    type Head = &'elem T;
    type Tail = &'elem [T];

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        if self.len() == 0 {
            None
        } else {
            self.split_first()
        }
    }
}

impl<'elem, T> Uncons for &'elem mut Vec<T> {
    type Head = &'elem mut T;
    type Tail = &'elem mut [T];

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        if self.len() == 0 {
            None
        } else {
            self.split_first_mut()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Void {}

impl List for Void {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nil;

impl List for Nil {}

impl<H> Cons<H> for Nil {
    type Output = Cell<H, Self>;

    fn cons(self, head: H) -> Self::Output {
        Cell { head, tail: self }
    }
}

impl Uncons for Nil {
    type Head = Void;
    type Tail = Void;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cell<H, T>
where
    T: List,
{
    pub head: H,
    pub tail: T,
}

impl<H, T> Cell<H, T>
where
    T: List,
{
    pub fn as_ref(&self) -> Cell<&H, &T> {
        Cell { head: &self.head, tail: &self.tail }
    }

    pub fn as_mut(&mut self) -> Cell<&mut H, &mut T> {
        Cell { head: &mut self.head, tail: &mut self.tail }
    }
}

impl<H, T> List for Cell<H, T> where T: List {}

impl<H1, H2, T> Cons<H2> for Cell<H1, T>
where
    T: List,
{
    type Output = Cell<H2, Self>;

    fn cons(self, head: H2) -> Self::Output {
        Cell { head, tail: self }
    }
}

impl<H, T> Uncons for Cell<H, T>
where
    T: List,
{
    type Head = H;
    type Tail = T;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        Some((self.head, self.tail))
    }
}

impl<'this, H, T> Uncons for &'this Cell<H, T>
where
    T: List,
{
    type Head = &'this H;
    type Tail = &'this T;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        self.as_ref().uncons()
    }
}

impl<'this, H, T> Uncons for &'this mut Cell<H, T>
where
    T: List,
{
    type Head = &'this mut H;
    type Tail = &'this mut T;

    fn uncons(self) -> Option<(Self::Head, Self::Tail)> {
        self.as_mut().uncons()
    }
}
