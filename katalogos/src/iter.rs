//! Tools for iterables over heterogenous elements.

/// A trait to make trait bounds more ergnomic relating to iteration by
/// reference.
pub trait IntoIterRef {
    /// Item of the iterator.
    type Item;

    /// Type of the iterator.
    type Iter<'item>: Iterator<Item = &'item Self::Item>
    where
        Self: 'item;

    /// Converts the reference into an iterator.
    fn iter<'item>(&'item self) -> Self::Iter<'item>;

    /// Assuming this iterable yields another iterable, this methods
    /// concatenates all elements into a flat iterator.
    fn concat(self) -> Concat<Self>
    where
        Self: Sized,
        Self::Item: IntoIterRef,
    {
        Concat(self)
    }
}

impl<T, I> IntoIterRef for T
where
    for<'this> &'this T: IntoIterator<Item = &'this I>,
{
    type Item = I;
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
    L::Item: IntoIterRef;

impl<'this, L> IntoIterator for &'this Concat<L>
where
    L: IntoIterRef,
    L::Item: IntoIterRef,
{
    type Item = &'this <L::Item as IntoIterRef>::Item;
    type IntoIter = ConcatIntoIter<'this, L>;

    fn into_iter(self) -> Self::IntoIter {
        let mut outer = self.0.iter();
        ConcatIntoIter { inner: outer.next().map(|item| item.iter()), outer }
    }
}

/// Iterator for [`Concat`].
pub struct ConcatIntoIter<'this, L>
where
    L: IntoIterRef + 'this,
    L::Item: IntoIterRef,
{
    outer: L::Iter<'this>,
    inner: Option<<L::Item as IntoIterRef>::Iter<'this>>,
}

impl<'this, L> Iterator for ConcatIntoIter<'this, L>
where
    L: IntoIterRef + 'this,
    L::Item: IntoIterRef,
{
    type Item = &'this <L::Item as IntoIterRef>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.as_mut()?.next() {
                Some(item) => break Some(item),
                None => self.inner = self.outer.next().map(|item| item.iter()),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::IntoIterRef;
    use crate::{harray, hiter, hvec};

    #[test]
    fn should_concat_correctly() {
        let elements = harray![harray![4, 5], hvec![7, 8, 9]]
            .concat()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
    }
}
