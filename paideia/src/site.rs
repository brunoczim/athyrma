//! This module provides a "filesystem-like" utility for organizing a site's
//! pages and effectively generating them.

use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::{
    component::{page::PageComponent, Component},
    location::{Fragment, InternalPath},
    render::{self, Context, Render, RenderAsDisplay},
};

/// An error that may happen when building a site.
#[derive(Debug)]
pub struct BuildError {
    /// Path of the problematic file.
    pub path: InternalPath,
    /// IO error that caused the build error.
    pub cause: io::Error,
}

impl From<BuildError> for io::Error {
    fn from(error: BuildError) -> Self {
        error.cause
    }
}

/// A site's filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site<P>
where
    P: Component<Kind = PageComponent>,
{
    /// Root directory.
    pub root: Directory<P>,
}

impl<P> Site<P>
where
    P: Component<Kind = PageComponent>,
{
    /// Builds the site into a concrete filesystem, given a render format,
    /// an output directory path, a resource directory path.
    ///
    /// The output and resource directories must be a mutable reference because
    /// they will be used to navigate to the site, but they will be restored,
    /// unless a panic occurs.
    pub fn build<W>(
        &self,
        format: &mut W,
        output_dir: &mut PathBuf,
        resource_dir: &mut PathBuf,
    ) -> Result<(), BuildError>
    where
        W: render::Format + ?Sized,
        P: Render<W>,
    {
        enum Operation<'site, P>
        where
            P: Component<Kind = PageComponent>,
        {
            Build(Entry<&'site P, &'site Directory<P>>),
            Push(&'site Fragment),
            Pop,
        }

        let dest = output_dir;
        let source = resource_dir;
        let mut internal_path = InternalPath::default();

        fs::remove_dir_all(&dest).map_err(|cause| BuildError {
            path: internal_path.clone(),
            cause,
        })?;

        let mut operations =
            vec![Operation::Build(Entry::Directory(&self.root))];

        while let Some(operation) = operations.pop() {
            match operation {
                Operation::Build(Entry::Directory(directory)) => {
                    fs::create_dir_all(&dest).map_err(|cause| BuildError {
                        path: internal_path.clone(),
                        cause,
                    })?;
                    for (fragment, entry) in &directory.entries {
                        operations.push(Operation::Pop);
                        operations.push(Operation::Build(entry.by_ref()));
                        operations.push(Operation::Push(fragment));
                    }
                },

                Operation::Build(Entry::Page(page)) => {
                    let mut file = fs::File::open(&dest).map_err(|cause| {
                        BuildError { path: internal_path.clone(), cause }
                    })?;

                    let context = Context::new(&internal_path, &PageComponent);
                    let renderer = RenderAsDisplay::new(page, format, context);

                    write!(file, "{}", renderer).map_err(|cause| {
                        BuildError { path: internal_path.clone(), cause }
                    })?;
                },

                Operation::Build(Entry::Resource) => {
                    fs::copy(&source, &dest).map_err(|cause| BuildError {
                        path: internal_path.clone(),
                        cause,
                    })?;
                },

                Operation::Push(fragment) => {
                    dest.push(fragment.as_str());
                    source.push(fragment.as_str());
                    internal_path.fragments.push(fragment.clone());
                },

                Operation::Pop => {
                    dest.pop();
                    source.pop();
                    internal_path.fragments.pop();
                },
            }
        }

        Ok(())
    }
}

/// A site's directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory<P>
where
    P: Component<Kind = PageComponent>,
{
    /// Entries of the directory, mapped by a name (i.e. a fragment of path).
    pub entries: HashMap<Fragment, Entry<P>>,
}

impl<P> Directory<P>
where
    P: Component<Kind = PageComponent>,
{
    /// Gets an entry from the directory given an accessor.
    pub fn get<'this, A>(&'this self, accessor: A) -> A::Output
    where
        A: Accessor<&'this Self>,
    {
        accessor.access(self)
    }

    /// Mutabily gets an entry from the directory given an accessor.
    pub fn get_mut<'this, A>(&'this mut self, accessor: A) -> A::Output
    where
        A: Accessor<&'this mut Self>,
    {
        accessor.access(self)
    }
}

/// An entry at a directory. Parametrized so pages and directories can be
/// replaced by references to them, e.g. `Entry<P, D> -> Entry<&P, &D>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry<P, D = Directory<P>>
where
    P: Component<Kind = PageComponent>,
{
    /// This entry is a page to be rendered.
    Page(P),
    /// This entry is a directory with more entries.
    Directory(D),
    /// This entry is an external resource.
    Resource,
}

impl<P, D> Entry<P, D>
where
    P: Component<Kind = PageComponent>,
{
    /// Is this entry a page?
    pub fn is_page(&self) -> bool {
        matches!(self, Self::Page(_))
    }

    /// Is this entry a directory?
    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    /// Is this entry a resource?
    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource)
    }

    /// Replaces this entry's data by references to them.
    pub fn by_ref(&self) -> Entry<&P, &D> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource => Entry::Resource,
        }
    }

    /// Replaces this entry's data by mutable references to them.
    pub fn by_mut(&mut self) -> Entry<&mut P, &mut D> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource => Entry::Resource,
        }
    }
}

/// An accessor over a directory.
pub trait Accessor<D> {
    /// The output value of such access.
    type Output;

    /// Access an entry of the directory.
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
    type Output = Option<Entry<&'dir P, &'dir Directory<P>>>;

    fn access(&self, directory: &'dir Directory<P>) -> Self::Output {
        let mut entry = Entry::Directory(directory);
        for fragment in &self.fragments {
            match entry {
                Entry::Page(_) => None?,
                Entry::Resource => None?,
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
    type Output = Option<Entry<&'dir mut P, &'dir mut Directory<P>>>;

    fn access(&self, directory: &'dir mut Directory<P>) -> Self::Output {
        let mut entry = Entry::Directory(directory);
        for fragment in &self.fragments {
            match entry {
                Entry::Page(_) => None?,
                Entry::Resource => None?,
                Entry::Directory(dir) => {
                    entry = dir.get_mut(fragment)?.by_mut()
                },
            }
        }
        Some(entry)
    }
}

#[cfg(test)]
mod test {
    use katalogos::harray;

    use crate::{
        component::{
            block::text::Paragraph,
            page::{Page, PageComponent},
        },
        location::{Fragment, InternalPath},
        render::FullRender,
    };

    use super::{Directory, Entry};

    fn make_directory() -> Directory<
        impl FullRender<Kind = PageComponent> + Eq + Send + Sync + 'static,
    > {
        Directory {
            entries: [
                (
                    Fragment::new("avocado").unwrap(),
                    Entry::Directory(Directory {
                        entries: [
                            (
                                Fragment::new("apple").unwrap(),
                                Entry::Page(Page {
                                    title: String::from("My Page"),
                                    assets: harray![],
                                    body: Paragraph("hello"),
                                    children: harray![],
                                }),
                            ),
                            (
                                Fragment::new("audio.ogg").unwrap(),
                                Entry::Resource,
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    }),
                ),
                (Fragment::new("pineapple").unwrap(), Entry::Resource),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn access_fragment_valid() {
        let dir = make_directory();
        assert!(dir
            .get(Fragment::new("avocado").unwrap())
            .unwrap()
            .is_directory());
    }

    #[test]
    fn access_fragment_invalid() {
        let dir = make_directory();
        assert!(dir.get(Fragment::new("grapes").unwrap()).is_none());
    }

    #[test]
    fn access_internal_path_valid() {
        let dir = make_directory();
        assert!(dir
            .get(InternalPath::parse("avocado/apple").unwrap())
            .unwrap()
            .is_page());
    }

    #[test]
    fn access_internal_path_invalid() {
        let dir = make_directory();
        assert!(dir
            .get(InternalPath::parse("avocado/grapes").unwrap())
            .is_none());
    }
}
