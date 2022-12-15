#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grammar<T, N> {
    pub terminals: Vec<T>,
    pub non_terminals: Vec<N>,
    pub starting_non_term: N,
    pub productions: Vec<Production<T, N>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol<T, N> {
    Terminal(T),
    NonTerm(N),
}

impl<T, N> Symbol<T, N> {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Self::NonTerm(_))
    }

    pub fn as_ref(&self) -> Symbol<&T, &N> {
        match self {
            Self::Terminal(term) => Symbol::Terminal(term),
            Self::NonTerm(nonterm) => Symbol::NonTerm(nonterm),
        }
    }

    pub fn as_mut(&mut self) -> Symbol<&mut T, &mut N> {
        match self {
            Self::Terminal(term) => Symbol::Terminal(term),
            Self::NonTerm(nonterm) => Symbol::NonTerm(nonterm),
        }
    }
}

impl<'term, 'nonterm, T, N> Symbol<&'term T, &'nonterm N> {
    pub fn cloned(&self) -> Symbol<T, N>
    where
        T: Clone,
        N: Clone,
    {
        match *self {
            Self::Terminal(term) => Symbol::Terminal(term.clone()),
            Self::NonTerm(nonterm) => Symbol::NonTerm(nonterm.clone()),
        }
    }

    pub fn copied(&self) -> Symbol<T, N>
    where
        T: Copy,
        N: Copy,
    {
        match *self {
            Self::Terminal(term) => Symbol::Terminal(*term),
            Self::NonTerm(nonterm) => Symbol::NonTerm(*nonterm),
        }
    }
}

impl<'term, 'nonterm, T, N> Symbol<&'term mut T, &'nonterm mut N> {
    pub fn cloned(&self) -> Symbol<T, N>
    where
        T: Clone,
        N: Clone,
    {
        match self {
            Self::Terminal(term) => Symbol::Terminal((*term).clone()),
            Self::NonTerm(nonterm) => Symbol::NonTerm((*nonterm).clone()),
        }
    }

    pub fn copied(&self) -> Symbol<T, N>
    where
        T: Copy,
        N: Copy,
    {
        match self {
            Self::Terminal(term) => Symbol::Terminal(**term),
            Self::NonTerm(nonterm) => Symbol::NonTerm(**nonterm),
        }
    }
}

impl<T, N> Default for Symbol<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::Terminal(T::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Production<T, N> {
    pub input: N,
    pub output: Vec<Symbol<T, N>>,
}

impl<T, N> Production<T, N> {
    pub fn replace<'prod, 'word>(
        &'prod self,
        word: &'word mut Vec<Symbol<T, N>>,
    ) -> Replacement<'prod, 'word, T, N>
    where
        T: Clone,
        N: Clone + Eq,
    {
        let back = word.len();
        Replacement { production: self, word, front: 0, back }
    }
}

#[derive(Debug)]
pub struct Replacement<'prod, 'word, T, N>
where
    T: Clone,
    N: Clone + Eq,
{
    production: &'prod Production<T, N>,
    word: &'word mut Vec<Symbol<T, N>>,
    front: usize,
    back: usize,
}

impl<'prod, 'word, T, N> Replacement<'prod, 'word, T, N>
where
    T: Clone,
    N: Clone + Eq,
{
    pub fn replace_next(&mut self) -> bool {
        match self.word[self.front .. self.back].iter().position(|symbol| {
            match symbol {
                Symbol::NonTerm(symbol) => self.production.input == *symbol,
                Symbol::Terminal(_) => false,
            }
        }) {
            Some(position) => {
                let start = self.front + position;
                let end = start + 1;
                self.word.splice(
                    start .. end,
                    self.production.output[..].iter().cloned(),
                );
                self.front = end;
                true
            },
            None => false,
        }
    }

    pub fn replace_back(&mut self) -> bool {
        match self.word[self.front .. self.back].iter().rposition(|symbol| {
            match symbol {
                Symbol::NonTerm(symbol) => self.production.input == *symbol,
                Symbol::Terminal(_) => false,
            }
        }) {
            Some(position) => {
                let start = self.front + position;
                let end = start + 1;
                self.word.splice(
                    start .. end,
                    self.production.output[..].iter().cloned(),
                );
                self.back = start;
                true
            },
            None => false,
        }
    }

    pub fn replace_all(&mut self) -> usize {
        let mut count = 0;
        while self.replace_next() {
            count += 1;
        }
        count
    }

    pub fn replace_all_rev(&mut self) -> usize {
        let mut count = 0;
        while self.replace_back() {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{Grammar, Production, Symbol};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum LambdaCalcTerm {
        Ident,
        Lambda,
        Dot,
        OpenParen,
        CloseParen,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum LambdaCalcNonTerm {
        Start,
        Var,
        App,
        Lambda,
        Expr,
    }

    pub fn lambda_calc_grammar() -> Grammar<LambdaCalcTerm, LambdaCalcNonTerm> {
        use LambdaCalcNonTerm::*;
        use LambdaCalcTerm::*;
        use Symbol::*;

        Grammar {
            terminals: vec![
                Ident,
                LambdaCalcTerm::Lambda,
                Dot,
                OpenParen,
                CloseParen,
            ],
            non_terminals: vec![
                Start,
                Var,
                App,
                LambdaCalcNonTerm::Lambda,
                Expr,
            ],
            starting_non_term: Start,
            productions: vec![
                Production { input: Start, output: vec![NonTerm(Expr)] },
                Production { input: Var, output: vec![Terminal(Ident)] },
                Production {
                    input: LambdaCalcNonTerm::Lambda,
                    output: vec![
                        Terminal(LambdaCalcTerm::Lambda),
                        Terminal(Ident),
                        Terminal(Dot),
                        NonTerm(Expr),
                    ],
                },
                Production {
                    input: App,
                    output: vec![NonTerm(Var), NonTerm(Expr)],
                },
                Production {
                    input: App,
                    output: vec![NonTerm(App), NonTerm(Expr)],
                },
                Production {
                    input: App,
                    output: vec![
                        Terminal(OpenParen),
                        NonTerm(Expr),
                        Terminal(CloseParen),
                        NonTerm(Expr),
                    ],
                },
                Production { input: Expr, output: vec![NonTerm(Var)] },
                Production {
                    input: Expr,
                    output: vec![NonTerm(LambdaCalcNonTerm::Lambda)],
                },
                Production { input: Expr, output: vec![NonTerm(App)] },
                Production {
                    input: Expr,
                    output: vec![
                        Terminal(OpenParen),
                        NonTerm(Expr),
                        Terminal(CloseParen),
                    ],
                },
            ],
        }
    }
}
