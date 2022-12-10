//! Tools for iterables over heterogenous elements.

/// A trait to make trait bounds more ergnomic relating to iteration by
/// reference.
pub trait IntoIterRef {
    /// Item of the iterator.
    type Item<'item>
    where
        Self: 'item;

    /// Type of the iterator.
    type Iter<'item>: Iterator<Item = Self::Item<'item>>
    where
        Self: 'item;

    /// Converts the reference into an iterator.
    fn iter<'item>(&'item self) -> Self::Iter<'item>;

    /// Assuming this iterable yields another iterable, this methods
    /// concatenates all elements into a flat iterator.
    fn concat(self) -> Concat<Self>
    where
        Self: Sized,
        for<'this> Self::Item<'this>: IntoIterator,
    {
        Concat(self)
    }
}

impl<T> IntoIterRef for T
where
    for<'this> &'this T: IntoIterator,
{
    type Item<'item> = <&'item T as IntoIterator>::Item where T: 'item;
    type Iter<'item> = <&'item T as IntoIterator>::IntoIter where T: 'item;

    fn iter<'item>(&'item self) -> Self::Iter<'item> {
        self.into_iter()
    }
}

/// Concatenates the elements of the given iterable, assuming they're iterable
/// as well, flattening the outer list.
pub struct Concat<L>(pub L)
where
    L: IntoIterRef,
    for<'item> L::Item<'item>: IntoIterator;

impl<'this, L> IntoIterator for &'this Concat<L>
where
    L: IntoIterRef,
    for<'item> L::Item<'item>: IntoIterator,
{
    type Item = <L::Item<'this> as IntoIterator>::Item;
    type IntoIter = ConcatIntoIter<'this, L>;

    fn into_iter(self) -> Self::IntoIter {
        let mut outer = self.0.iter();
        ConcatIntoIter {
            inner: outer.next().map(|item| item.into_iter()),
            outer,
        }
    }
}

/// Iterator for [`Concat`].
pub struct ConcatIntoIter<'item, L>
where
    L: 'item,
    L: IntoIterRef,
    L::Item<'item>: IntoIterator,
{
    outer: L::Iter<'item>,
    inner: Option<<L::Item<'item> as IntoIterator>::IntoIter>,
}

impl<'item, L> Iterator for ConcatIntoIter<'item, L>
where
    L: IntoIterRef,
    L::Item<'item>: IntoIterator,
{
    type Item = <L::Item<'item> as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.as_mut()?.next() {
                Some(item) => break Some(item),
                None => {
                    self.inner = self.outer.next().map(|item| item.into_iter())
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::IntoIterRef;
    use crate::{harray, hvec};

    /*
    #[test]
    fn should_concat_correctly() {
        let elements =
            harray![harray![1, 2, 3], harray![5, 7], hvec![12, 8, 9]]
                .concat()
                .iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<_>>();

        assert_eq!(elements, &["1", "2", "3", "5", "7", "12", "8", "9"]);
    }
    */
}
