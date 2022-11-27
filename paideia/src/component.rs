pub mod inline;
pub mod block;
pub mod section;
pub mod asset;
pub mod page;

use katalogos::coproduct::{Cocons, Conil};
use std::{fmt, rc::Rc, sync::Arc};

pub use block::BlockComponent;
pub use inline::InlineComponent;

pub trait ComponentKind {}

pub trait Component: fmt::Debug {
    type Kind: ComponentKind + ?Sized;
}

impl<'this, T> Component for &'this T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<'this, T> Component for &'this mut T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Box<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Rc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Arc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<C> Component for Conil<C>
where
    C: ComponentKind,
{
    type Kind = C;
}

impl<H, T> Component for Cocons<H, T>
where
    H: Component,
    T: Component<Kind = H::Kind>,
{
    type Kind = H::Kind;
}

impl<T, const N: usize> Component for [T; N]
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<T> Component for [T]
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<T> Component for Vec<T>
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<A, B> Component for (A, B)
where
    A: Component,
    B: Component<Kind = A::Kind>,
{
    type Kind = A::Kind;
}

impl<'this, K> ComponentKind for &'this K where K: ComponentKind + ?Sized {}

impl<'this, K> ComponentKind for &'this mut K where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Box<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Rc<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Arc<K> where K: ComponentKind + ?Sized {}
