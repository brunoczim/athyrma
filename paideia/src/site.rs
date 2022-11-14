use std::{collections::HashMap, path::PathBuf};

use crate::{
    component::{page::PageComponent, Component},
    location::{Fragment, InternalPath},
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

impl<P> Directory<P>
where
    P: Component<Kind = PageComponent>,
{
    pub fn get<'this, A>(&'this self, accessor: A) -> A::Output
    where
        A: Accessor<&'this Self>,
    {
        accessor.access(self)
    }

    pub fn get_mut<'this, A>(&'this mut self, accessor: A) -> A::Output
    where
        A: Accessor<&'this mut Self>,
    {
        accessor.access(self)
    }
}

#[derive(Debug, Clone)]
pub enum Entry<P, D = Directory<P>, R = PathBuf>
where
    P: Component<Kind = PageComponent>,
{
    Page(P),
    Directory(D),
    Resource(R),
}

impl<P, D, R> Entry<P, D, R>
where
    P: Component<Kind = PageComponent>,
{
    pub fn by_ref(&self) -> Entry<&P, &D, &R> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource(resource) => Entry::Resource(resource),
        }
    }

    pub fn by_mut(&mut self) -> Entry<&mut P, &mut D, &mut R> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource(resource) => Entry::Resource(resource),
        }
    }
}

pub trait Accessor<D> {
    type Output;

    fn access(&self, directory: D) -> Self::Output;
}

impl<'this, A, D> Accessor<D> for &'this A
where
    A: Accessor<D> + ?Sized,
{
    type Output = A::Output;

    fn access(&self, directory: D) -> Self::Output {
        (**self).access(directory)
    }
}

impl<'dir, P> Accessor<&'dir Directory<P>> for Fragment
where
    P: Component<Kind = PageComponent>,
{
    type Output = Option<&'dir Entry<P>>;

    fn access(&self, directory: &'dir Directory<P>) -> Self::Output {
        directory.entries.get(self)
    }
}

impl<'dir, P> Accessor<&'dir mut Directory<P>> for Fragment
where
    P: Component<Kind = PageComponent>,
{
    type Output = Option<&'dir mut Entry<P>>;

    fn access(&self, directory: &'dir mut Directory<P>) -> Self::Output {
        directory.entries.get_mut(self)
    }
}

impl<'dir, P> Accessor<&'dir Directory<P>> for InternalPath
where
    P: Component<Kind = PageComponent>,
{
    type Output = Option<Entry<&'dir P, &'dir Directory<P>, &'dir PathBuf>>;

    fn access(&self, directory: &'dir Directory<P>) -> Self::Output {
        let mut entry = Entry::Directory(directory);
        for fragment in &self.fragments {
            match entry {
                Entry::Page(_) => None?,
                Entry::Resource(_) => None?,
                Entry::Directory(dir) => entry = dir.get(fragment)?.by_ref(),
            }
        }
        Some(entry)
    }
}

impl<'dir, P> Accessor<&'dir mut Directory<P>> for InternalPath
where
    P: Component<Kind = PageComponent>,
{
    type Output =
        Option<Entry<&'dir mut P, &'dir mut Directory<P>, &'dir mut PathBuf>>;

    fn access(&self, directory: &'dir mut Directory<P>) -> Self::Output {
        let mut entry = Entry::Directory(directory);
        for fragment in &self.fragments {
            match entry {
                Entry::Page(_) => None?,
                Entry::Resource(_) => None?,
                Entry::Directory(dir) => {
                    entry = dir.get_mut(fragment)?.by_mut()
                },
            }
        }
        Some(entry)
    }
}
