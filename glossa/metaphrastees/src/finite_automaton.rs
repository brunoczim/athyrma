pub trait FiniteAutomaton<T> {
    fn test<'item, I>(&self, input: I) -> bool
    where
        T: 'item,
        I: IntoIterator<Item = &'item T>;
}

pub trait Enumerable {
    fn count() -> usize;

    fn from_index(index: usize) -> Self;

    fn to_index(&self) -> usize;
}
