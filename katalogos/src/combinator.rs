use crate::{
    coproduct::{Cocons, Conil},
    function::FunctionMut,
};

pub trait Map<F> {
    type Output;

    fn map(self, mapper: F) -> Self::Output;
}

impl<M, F> Map<F> for Conil<M>
where
    M: ?Sized,
{
    type Output = Self;

    fn map(self, _mapper: F) -> Self::Output {
        self
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

pub trait FoldLeft<A, F> {
    fn fold_left(self, accumulator: A, folder: F) -> A;
}

impl<M, A, F> FoldLeft<A, F> for Conil<M>
where
    M: ?Sized,
{
    fn fold_left(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<H, T, A, F> FoldLeft<A, F> for Cocons<H, T>
where
    F: FunctionMut<(A, H), Output = A>,
    T: FoldLeft<A, F>,
{
    fn fold_left(self, accumulator: A, mut folder: F) -> A {
        match self {
            Cocons::Head(head) => folder.call_mut((accumulator, head)),
            Cocons::Tail(tail) => tail.fold_left(accumulator, folder),
        }
    }
}

pub trait FoldRight<A, F> {
    fn fold_right(self, accumulator: A, folder: F) -> A;
}

impl<M, A, F> FoldRight<A, F> for Conil<M>
where
    M: ?Sized,
{
    fn fold_right(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<H, T, A, F> FoldRight<A, F> for Cocons<H, T>
where
    F: FunctionMut<(H, A), Output = A>,
    T: FoldRight<A, F>,
{
    fn fold_right(self, accumulator: A, mut folder: F) -> A {
        match self {
            Cocons::Head(head) => folder.call_mut((head, accumulator)),
            Cocons::Tail(tail) => tail.fold_right(accumulator, folder),
        }
    }
}
