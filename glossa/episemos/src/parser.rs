use crate::grammar::{Grammar, Symbol};
use std::collections::{btree_set, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxTree<T, N> {
    Leaf(SyntaxTreeLeaf<T>),
    Branch(SyntaxTreeBranch<T, N>),
}

impl<T, N> Default for SyntaxTree<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::Leaf(SyntaxTreeLeaf::default())
    }
}

impl<T, N> SyntaxTree<T, N> {
    pub fn top_symbol(&self) -> Symbol<&T, &N> {
        match self {
            Self::Leaf(node) => Symbol::Terminal(&node.symbol),
            Self::Branch(node) => Symbol::NonTerm(&node.label),
        }
    }

    pub fn count_leaves(&self) -> usize {
        match self {
            Self::Leaf(node) => node.count_leaves(),
            Self::Branch(node) => node.count_leaves(),
        }
    }
}

impl<'term, 'nonterm, T, N> SyntaxTree<&'term T, &'nonterm N> {
    pub fn cloned(&self) -> SyntaxTree<T, N>
    where
        T: Clone,
        N: Clone,
    {
        match self {
            Self::Leaf(node) => SyntaxTree::Leaf(node.cloned()),
            Self::Branch(node) => SyntaxTree::Branch(node.cloned()),
        }
    }
}

impl<'term, 'nonterm, T, N> SyntaxTree<&'term mut T, &'nonterm mut N> {
    pub fn cloned(&self) -> SyntaxTree<T, N>
    where
        T: Clone,
        N: Clone,
    {
        match self {
            Self::Leaf(node) => SyntaxTree::Leaf(node.cloned()),
            Self::Branch(node) => SyntaxTree::Branch(node.cloned()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SyntaxTreeLeaf<T> {
    pub symbol: T,
}

impl<T> SyntaxTreeLeaf<T> {
    pub fn count_leaves(&self) -> usize {
        1
    }
}

impl<'sym, T> SyntaxTreeLeaf<&'sym T> {
    pub fn cloned(&self) -> SyntaxTreeLeaf<T>
    where
        T: Clone,
    {
        SyntaxTreeLeaf { symbol: self.symbol.clone() }
    }
}

impl<'sym, T> SyntaxTreeLeaf<&'sym mut T> {
    pub fn cloned(&self) -> SyntaxTreeLeaf<T>
    where
        T: Clone,
    {
        SyntaxTreeLeaf { symbol: self.symbol.clone() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SyntaxTreeBranch<T, N> {
    pub label: N,
    pub children: Vec<SyntaxTree<T, N>>,
}

impl<T, N> SyntaxTreeBranch<T, N> {
    pub fn count_leaves(&self) -> usize {
        self.children.iter().map(|child| child.count_leaves()).sum()
    }
}

impl<'term, 'nonterm, T, N> SyntaxTreeBranch<&'term T, &'nonterm N> {
    pub fn cloned(&self) -> SyntaxTreeBranch<T, N>
    where
        T: Clone,
        N: Clone,
    {
        SyntaxTreeBranch {
            label: self.label.clone(),
            children: self
                .children
                .iter()
                .map(|child| child.cloned())
                .collect(),
        }
    }
}

impl<'term, 'nonterm, T, N> SyntaxTreeBranch<&'term mut T, &'nonterm mut N> {
    pub fn cloned(&self) -> SyntaxTreeBranch<T, N>
    where
        T: Clone,
        N: Clone,
    {
        SyntaxTreeBranch {
            label: self.label.clone(),
            children: self
                .children
                .iter()
                .map(|child| child.cloned())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Candidate<T, N> {
    start: usize,
    length: usize,
    syntax_tree: SyntaxTree<T, N>,
}

impl<T, N> Candidate<T, N> {
    pub fn from_syntax_tree(
        start: usize,
        syntax_tree: SyntaxTree<T, N>,
    ) -> Self {
        Self { start, length: syntax_tree.count_leaves(), syntax_tree }
    }

    pub fn from_leaf(start: usize, symbol: T) -> Self {
        Self {
            start,
            length: 1,
            syntax_tree: SyntaxTree::Leaf(SyntaxTreeLeaf { symbol }),
        }
    }

    pub fn from_branch<I>(start: usize, label: N, children: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut branch = SyntaxTreeBranch { label, children: Vec::new() };
        let mut length = 0;
        for candidate in children {
            length += candidate.length;
            branch.children.push(candidate.syntax_tree);
        }
        Self { start, length, syntax_tree: SyntaxTree::Branch(branch) }
    }
}

impl<T, N> Default for Candidate<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::from_leaf(0, T::default())
    }
}

#[derive(Debug, Clone)]
struct Parser<'grammar, T, N> {
    input_length: usize,
    grammar: &'grammar Grammar<T, N>,
    candidates: BTreeSet<Candidate<&'grammar T, &'grammar N>>,
}

impl<'grammar, T, N> Parser<'grammar, T, N>
where
    T: Ord,
    N: Ord,
{
    fn new(grammar: &'grammar Grammar<T, N>, input: &[T]) -> Self {
        Self {
            grammar,
            input_length: input.len(),
            candidates: input
                .iter()
                .enumerate()
                .filter_map(|(start, left)| {
                    grammar
                        .terminals
                        .iter()
                        .find(|right| left == *right)
                        .map(|symbol| Candidate::from_leaf(start, symbol))
                })
                .collect(),
        }
    }

    fn solutions(self) -> Solutions<'grammar, T, N> {
        Solutions {
            input_length: self.input_length,
            grammar: self.grammar,
            candidates: self.candidates.into_iter(),
        }
    }

    fn derive(&mut self) {
        while self.iterate() {}
    }

    fn iterate(&mut self) -> bool {
        let mut found = false;

        let grammar = self.grammar;
        for production in &grammar.productions {
            let mut candidates = self.candidates.clone();
            for start in 0 .. self.input_length {
                candidates.retain(|candidate| candidate.start >= start);
                let mut children = Vec::<Candidate<&T, &N>>::new();

                loop {
                    let matches =
                        production.output.iter().map(Symbol::as_ref).eq(
                            children.iter().map(|child| {
                                child.syntax_tree.top_symbol().copied()
                            }),
                        );

                    if matches
                        && self.candidates.insert(Candidate::from_branch(
                            start,
                            &production.input,
                            children.iter().cloned(),
                        ))
                    {
                        found = true;
                        break;
                    }

                    // TODO replace for .first() when stable
                    match candidates.iter().next() {
                        Some(child) => {
                            children.push(child.clone());
                        },
                        None => break,
                    }
                }
            }
        }

        found
    }
}

#[derive(Debug)]
pub struct Solutions<'grammar, T, N>
where
    T: Ord,
    N: Ord,
{
    input_length: usize,
    grammar: &'grammar Grammar<T, N>,
    candidates: btree_set::IntoIter<Candidate<&'grammar T, &'grammar N>>,
}

impl<'grammar, T, N> Iterator for Solutions<'grammar, T, N>
where
    T: Ord,
    N: Ord,
{
    type Item = SyntaxTree<&'grammar T, &'grammar N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let candidate = self.candidates.next()?;
            if candidate.start == 0
                && candidate.length == self.input_length
                && candidate.syntax_tree.top_symbol().copied()
                    == Symbol::NonTerm(&self.grammar.starting_non_term)
            {
                break Some(candidate.syntax_tree);
            }
        }
    }
}

pub fn parse<'grammar, T, N>(
    grammar: &'grammar Grammar<T, N>,
    input: &[T],
) -> Solutions<'grammar, T, N>
where
    T: Ord,
    N: Ord,
{
    let mut parser = Parser::new(grammar, input);
    parser.derive();
    parser.solutions()
}
