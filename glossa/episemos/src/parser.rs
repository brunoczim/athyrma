use crate::grammar::Grammar;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxTree<T, N> {
    Leaft(SyntaxTreeLeaf<T>),
    Branch(SyntaxTreeBranch<T, N>),
}

impl<T, N> Default for SyntaxTree<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::Leaft(SyntaxTreeLeaf::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SyntaxTreeLeaf<T> {
    pub symbol: T,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SyntaxTreeBranch<T, N> {
    pub label: N,
    pub children: Vec<SyntaxTree<T, N>>,
}

pub fn parse<T, N>(
    grammar: &Grammar<T, N>,
    input: &[T],
) -> Option<SyntaxTree<T, N>> {
    None
}
