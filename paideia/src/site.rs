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

#[derive(Debug)]
pub struct BuildError {
    pub path: InternalPath,
    pub cause: io::Error,
}

impl From<BuildError> for io::Error {
    fn from(error: BuildError) -> Self {
        error.cause
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site<P>
where
    P: Component<Kind = PageComponent>,
{
    pub root: Directory<P>,
}

impl<P> Site<P>
where
    P: Component<Kind = PageComponent>,
{
    pub fn build<W>(
        &self,
        format: &mut W,
        parent: &mut PathBuf,
        resources: &mut PathBuf,
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

        let dest = parent;
        let source = resources;
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry<P, D = Directory<P>>
where
    P: Component<Kind = PageComponent>,
{
    Page(P),
    Directory(D),
    Resource,
}

impl<P, D> Entry<P, D>
where
    P: Component<Kind = PageComponent>,
{
    pub fn is_page(&self) -> bool {
        matches!(self, Self::Page(_))
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource)
    }

    pub fn by_ref(&self) -> Entry<&P, &D> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource => Entry::Resource,
        }
    }

    pub fn by_mut(&mut self) -> Entry<&mut P, &mut D> {
        match self {
            Self::Page(page) => Entry::Page(page),
            Self::Directory(dir) => Entry::Directory(dir),
            Self::Resource => Entry::Resource,
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
    use katalogos::{hlist, HList};

    use crate::{
        component::{
            asset::AssetComponent,
            block::text::Paragraph,
            page::{Page, PageComponent},
            section::SectionComponent,
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
                                Entry::Page(Page::<
                                    HList![(): AssetComponent],
                                    _,
                                    HList![(): SectionComponent],
                                > {
                                    title: String::from("My Page"),
                                    assets: hlist![],
                                    body: Paragraph("hello"),
                                    children: hlist![],
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
