use crate::{
    colist::{Cocons, Conil},
    function::FunctionMut,
    list::{Cons, Nil},
};

pub trait Map<F> {
    type Output;

    fn map(self, mapper: F) -> Self::Output;
}

impl<A, F> Map<F> for Nil<A> {
    type Output = Self;

    fn map(self, _mapper: F) -> Self::Output {
        self
    }
}

impl<A, F> Map<F> for Conil<A> {
    type Output = Self;

    fn map(self, _mapper: F) -> Self::Output {
        self
    }
}

impl<H, T, F> Map<F> for Cons<H, T>
where
    T: Map<F>,
    F: FunctionMut<H>,
{
    type Output = Cons<F::Output, T::Output>;

    fn map(self, mut mapper: F) -> Self::Output {
        Cons { head: mapper.call_mut(self.head), tail: self.tail.map(mapper) }
    }
}

impl<H, T, F> Map<F> for Cocons<H, T>
where
    T: Map<F>,
    F: FunctionMut<H>,
{
    type Output = Cocons<F::Output, T::Output>;

    fn map(self, mut mapper: F) -> Self::Output {
        match self {
            Cocons::Head(head) => Cocons::Head(mapper.call_mut(head)),
            Cocons::Tail(tail) => Cocons::Tail(tail.map(mapper)),
        }
    }
}

pub trait FoldLeft<B, F> {
    fn fold_left(self, accumulator: B, folder: F) -> B;
}

impl<A, B, F> FoldLeft<B, F> for Nil<A> {
    fn fold_left(self, accumulator: B, _folder: F) -> B {
        accumulator
    }
}

impl<A, B, F> FoldLeft<B, F> for Conil<A> {
    fn fold_left(self, accumulator: B, _folder: F) -> B {
        accumulator
    }
}

impl<H, T, B, F> FoldLeft<B, F> for Cons<H, T>
where
    F: FunctionMut<(B, H), Output = B>,
    T: FoldLeft<B, F>,
{
    fn fold_left(self, accumulator: B, mut folder: F) -> B {
        self.tail.fold_left(folder.call_mut((accumulator, self.head)), folder)
    }
}

impl<H, T, B, F> FoldLeft<B, F> for Cocons<H, T>
where
    F: FunctionMut<(B, H), Output = B>,
    T: FoldLeft<B, F>,
{
    fn fold_left(self, accumulator: B, mut folder: F) -> B {
        match self {
            Cocons::Head(head) => folder.call_mut((accumulator, head)),
            Cocons::Tail(tail) => tail.fold_left(accumulator, folder),
        }
    }
}

pub trait FoldRight<B, F> {
    fn fold_right(self, accumulator: B, folder: F) -> B;
}

impl<A, B, F> FoldRight<B, F> for Nil<A> {
    fn fold_right(self, accumulator: B, _folder: F) -> B {
        accumulator
    }
}

impl<A, B, F> FoldRight<B, F> for Conil<A> {
    fn fold_right(self, accumulator: B, _folder: F) -> B {
        accumulator
    }
}

impl<H, T, B, F> FoldRight<B, F> for Cons<H, T>
where
    F: FunctionMut<(H, B), Output = B>,
    T: for<'a> FoldRight<B, &'a mut F>,
{
    fn fold_right(self, accumulator: B, mut folder: F) -> B {
        let new_accumulator = self.tail.fold_right(accumulator, &mut folder);
        folder.call_mut((self.head, new_accumulator))
    }
}

impl<H, T, B, F> FoldRight<B, F> for Cocons<H, T>
where
    F: FunctionMut<(H, B), Output = B>,
    T: FoldRight<B, F>,
{
    fn fold_right(self, accumulator: B, mut folder: F) -> B {
        match self {
            Cocons::Head(head) => folder.call_mut((head, accumulator)),
            Cocons::Tail(tail) => tail.fold_right(accumulator, folder),
        }
    }
}
