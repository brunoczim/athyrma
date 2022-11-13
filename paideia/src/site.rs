use std::collections::HashMap;

use crate::{
    component::{page::PageComponent, Component},
    location::Fragment,
};

#[derive(Debug, Clone)]
pub struct Site<P>
where
    P: Component<Kind = PageComponent>,
{
    pub root: Directory<P>,
}

#[derive(Debug, Clone)]
pub struct Directory<P>
where
    P: Component<Kind = PageComponent>,
{
    pub entries: HashMap<Fragment, Entry<P>>,
}

#[derive(Debug, Clone)]
pub enum Entry<P>
where
    P: Component<Kind = PageComponent>,
{
    Page(P),
    Directory(Directory<P>),
}

pub trait Accessor<D> {
    type Output;

    fn access(&self, directory: D) -> Self::Output;
}
