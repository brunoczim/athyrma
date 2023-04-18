pub trait FiniteAutomaton<T> {
    fn test<'item, I>(&self, input: I) -> bool
    where
        T: 'item,
        I: IntoIterator<Item = &'item T>;
}
