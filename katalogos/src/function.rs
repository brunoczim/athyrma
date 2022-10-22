pub trait FunctionOnce<M> {
    type Output;

    fn call_once(self, argument: M) -> Self::Output;
}

impl<'this, F, M> FunctionOnce<M> for &'this F
where
    F: Function<M> + ?Sized,
{
    type Output = F::Output;

    fn call_once(self, argument: M) -> Self::Output {
        self.call(argument)
    }
}

impl<'this, F, M> FunctionOnce<M> for &'this mut F
where
    F: FunctionMut<M> + ?Sized,
{
    type Output = F::Output;

    fn call_once(self, argument: M) -> Self::Output {
        self.call_mut(argument)
    }
}

pub trait FunctionMut<M>: FunctionOnce<M> {
    fn call_mut(&mut self, argument: M) -> Self::Output;
}

impl<'this, F, M> FunctionMut<M> for &'this F
where
    F: Function<M> + ?Sized,
{
    fn call_mut(&mut self, argument: M) -> Self::Output {
        (**self).call(argument)
    }
}

impl<'this, F, M> FunctionMut<M> for &'this mut F
where
    F: FunctionMut<M> + ?Sized,
{
    fn call_mut(&mut self, argument: M) -> Self::Output {
        (**self).call_mut(argument)
    }
}

pub trait Function<M>: FunctionMut<M> {
    fn call(&self, argument: M) -> Self::Output;
}

impl<'this, F, M> Function<M> for &'this F
where
    F: Function<M> + ?Sized,
{
    fn call(&self, argument: M) -> Self::Output {
        (**self).call(argument)
    }
}

impl<'this, F, M> Function<M> for &'this mut F
where
    F: Function<M> + ?Sized,
{
    fn call(&self, argument: M) -> Self::Output {
        (**self).call(argument)
    }
}
