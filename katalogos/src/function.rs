pub trait FunctionOnce<A> {
    type Output;

    fn call_once(self, argument: A) -> Self::Output;
}

impl<'this, F, A> FunctionOnce<A> for &'this F
where
    F: Function<A> + ?Sized,
{
    type Output = F::Output;

    fn call_once(self, argument: A) -> Self::Output {
        self.call(argument)
    }
}

impl<'this, F, A> FunctionOnce<A> for &'this mut F
where
    F: FunctionMut<A> + ?Sized,
{
    type Output = F::Output;

    fn call_once(self, argument: A) -> Self::Output {
        self.call_mut(argument)
    }
}

pub trait FunctionMut<A>: FunctionOnce<A> {
    fn call_mut(&mut self, argument: A) -> Self::Output;
}

impl<'this, F, A> FunctionMut<A> for &'this F
where
    F: Function<A> + ?Sized,
{
    fn call_mut(&mut self, argument: A) -> Self::Output {
        (**self).call(argument)
    }
}

impl<'this, F, A> FunctionMut<A> for &'this mut F
where
    F: FunctionMut<A> + ?Sized,
{
    fn call_mut(&mut self, argument: A) -> Self::Output {
        (**self).call_mut(argument)
    }
}

pub trait Function<A>: FunctionMut<A> {
    fn call(&self, argument: A) -> Self::Output;
}

impl<'this, F, A> Function<A> for &'this F
where
    F: Function<A> + ?Sized,
{
    fn call(&self, argument: A) -> Self::Output {
        (**self).call(argument)
    }
}

impl<'this, F, A> Function<A> for &'this mut F
where
    F: Function<A> + ?Sized,
{
    fn call(&self, argument: A) -> Self::Output {
        (**self).call(argument)
    }
}
