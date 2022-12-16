use crate::grammar::{Grammar, Symbol};
use std::{
    collections::{btree_set, BTreeSet},
    fmt,
};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Attempt<T, N> {
    start: usize,
    length: usize,
    children: Vec<SyntaxTree<T, N>>,
}

impl<T, N> Candidate<T, N> {
    pub fn from_leaf(start: usize, symbol: T) -> Self {
        Self {
            start,
            length: 1,
            syntax_tree: SyntaxTree::Leaf(SyntaxTreeLeaf { symbol }),
        }
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
    T: fmt::Debug,
    N: fmt::Debug,
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

    fn attempt_symbol(
        attempts: &mut BTreeSet<Attempt<&'grammar T, &'grammar N>>,
        candidate: &Candidate<&'grammar T, &'grammar N>,
    ) {
        let mut additional_attempts = BTreeSet::new();
        for attempt in &*attempts {
            if attempt.start + attempt.length == candidate.start {
                let mut attempt = attempt.clone();
                attempt.length += candidate.length;
                attempt.children.push(candidate.syntax_tree.clone());
                additional_attempts.insert(attempt);
            }
        }
        for attempt in additional_attempts {
            attempts.insert(attempt);
        }
        attempts.insert(Attempt {
            start: candidate.start,
            length: candidate.length,
            children: vec![candidate.syntax_tree.clone()],
        });
    }

    fn iterate(&mut self) -> bool {
        let mut attempts = BTreeSet::new();
        for candidate in &self.candidates {
            Self::attempt_symbol(&mut attempts, candidate);
        }

        let mut found = false;
        for production in &self.grammar.productions {
            for attempt in &attempts {
                if attempt
                    .children
                    .iter()
                    .map(|tree| tree.top_symbol().copied())
                    .eq(production.output.iter().map(|symbol| symbol.as_ref()))
                {
                    found = found
                        || self.candidates.insert(Candidate {
                            start: attempt.start,
                            length: attempt.length,
                            syntax_tree: SyntaxTree::Branch(SyntaxTreeBranch {
                                label: &production.input,
                                children: attempt.children.clone(),
                            }),
                        });
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

/// HORRIBLE, at best Omega(n^3)
pub fn parse<'grammar, T, N>(
    grammar: &'grammar Grammar<T, N>,
    input: &[T],
) -> Solutions<'grammar, T, N>
where
    T: Ord,
    N: Ord,
    T: fmt::Debug,
    N: fmt::Debug,
{
    let mut parser = Parser::new(grammar, input);
    parser.derive();
    parser.solutions()
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::{
        grammar::test::{
            lambda_calc_grammar,
            LambdaCalcNonTerm,
            LambdaCalcTerm,
        },
        parser::{SyntaxTree, SyntaxTreeBranch, SyntaxTreeLeaf},
    };

    #[test]
    fn simple_lambda_calc() {
        use LambdaCalcNonTerm::*;
        use LambdaCalcTerm::*;
        use SyntaxTree::*;

        let grammar = lambda_calc_grammar();
        let solution_iter = parse(&grammar, &[OpenParen, Ident, CloseParen]);
        let solutions =
            solution_iter.map(|solution| solution.cloned()).collect::<Vec<_>>();
        assert_eq!(solutions, &[Branch(SyntaxTreeBranch {
            label: Start,
            children: vec![Branch(SyntaxTreeBranch {
                label: Expr,
                children: vec![
                    Leaf(SyntaxTreeLeaf { symbol: OpenParen }),
                    Branch(SyntaxTreeBranch {
                        label: Expr,
                        children: vec![Branch(SyntaxTreeBranch {
                            label: Var,
                            children: vec![Leaf(SyntaxTreeLeaf {
                                symbol: Ident,
                            })],
                        })],
                    }),
                    Leaf(SyntaxTreeLeaf { symbol: CloseParen }),
                ]
            })],
        })]);
    }

    #[test]
    fn lambda_lambda_calc() {
        use LambdaCalcNonTerm::*;
        use LambdaCalcTerm::*;
        use SyntaxTree::*;

        let grammar = lambda_calc_grammar();
        let solution_iter = parse(&grammar, &[
            OpenParen,
            LambdaCalcTerm::Lambda,
            Ident,
            Dot,
            Ident,
            Ident,
            CloseParen,
            Ident,
        ]);
        let solutions =
            solution_iter.map(|solution| solution.cloned()).collect::<Vec<_>>();

        assert_eq!(solutions, &[Branch(SyntaxTreeBranch {
            label: Start,
            children: vec![Branch(SyntaxTreeBranch {
                label: Expr,
                children: vec![Branch(SyntaxTreeBranch {
                    label: App,
                    children: vec![
                        Leaf(SyntaxTreeLeaf { symbol: OpenParen }),
                        Branch(SyntaxTreeBranch {
                            label: Expr,
                            children: vec![Branch(SyntaxTreeBranch {
                                label: LambdaCalcNonTerm::Lambda,
                                children: vec![
                                    Leaf(SyntaxTreeLeaf {
                                        symbol: LambdaCalcTerm::Lambda,
                                    }),
                                    Leaf(SyntaxTreeLeaf { symbol: Ident }),
                                    Leaf(SyntaxTreeLeaf { symbol: Dot }),
                                    Branch(SyntaxTreeBranch {
                                        label: Expr,
                                        children: vec![Branch(
                                            SyntaxTreeBranch {
                                                label: App,
                                                children: vec![
                                                    Branch(SyntaxTreeBranch {
                                                        label: Var,
                                                        children: vec![Leaf(
                                                            SyntaxTreeLeaf {
                                                                symbol: Ident,
                                                            }
                                                        )],
                                                    }),
                                                    Branch(SyntaxTreeBranch {
                                                        label: Expr,
                                                        children: vec![Branch(
                                                            SyntaxTreeBranch {
                                                                label: Var,
                                                                children:
                                                                    vec![Leaf(
                                                            SyntaxTreeLeaf {
                                                                symbol: Ident,
                                                            }
                                                        )],
                                                            }
                                                        )],
                                                    }),
                                                ]
                                            }
                                        )],
                                    }),
                                ],
                            })],
                        }),
                        Leaf(SyntaxTreeLeaf { symbol: CloseParen }),
                        Branch(SyntaxTreeBranch {
                            label: Expr,
                            children: vec![Branch(SyntaxTreeBranch {
                                label: Var,
                                children: vec![Leaf(SyntaxTreeLeaf {
                                    symbol: Ident,
                                })],
                            })],
                        }),
                    ],
                }),]
            })],
        })]);
    }
}
