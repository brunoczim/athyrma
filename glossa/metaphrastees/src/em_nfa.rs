use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(pub u128);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionOutput<T>
where
    T: Hash + Ord,
{
    pub empty: BTreeSet<State>,
    pub symbols: HashMap<T, BTreeSet<State>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Automaton<T>
where
    T: Hash + Ord,
{
    pub initial_state: State,
    pub final_states: HashSet<State>,
    pub transitions: HashMap<State, TransitionOutput<T>>,
}

impl<T> Automaton<T>
where
    T: Hash + Ord,
{
    pub fn start(&self) -> Execution<T> {
        let mut execution = Execution {
            automaton: self,
            current_states: HashSet::from([self.initial_state]),
        };
        execution.next_empty_moves();
        execution
    }

    pub fn test<'item, I>(&self, input: I) -> bool
    where
        I: IntoIterator<Item = &'item T>,
        T: 'item,
    {
        let mut execution = self.start();
        for symbol in input {
            execution.next(symbol);
        }
        execution
            .current_states()
            .iter()
            .any(|state| self.final_states.contains(state))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Execution<'automaton, T>
where
    T: Hash + Ord,
{
    automaton: &'automaton Automaton<T>,
    current_states: HashSet<State>,
}

impl<'automaton, T> Execution<'automaton, T>
where
    T: Hash + Ord,
{
    pub fn current_states(&self) -> &HashSet<State> {
        &self.current_states
    }

    pub fn next(&mut self, symbol: &T) {
        let current_states = self.current_states.drain().collect::<Vec<_>>();

        for current_state in current_states {
            if let Some(next_states) =
                self.automaton.transitions.get(&current_state)
            {
                if let Some(next_for_symbol) = next_states.symbols.get(symbol) {
                    for &next_state in next_for_symbol {
                        self.current_states.insert(next_state);
                    }
                }
            }
        }

        self.next_empty_moves();
    }

    fn next_empty_moves(&mut self) {
        let mut prev_state_count = self.current_states.len();
        loop {
            for current_state in self.current_states.clone() {
                if let Some(next_states) =
                    self.automaton.transitions.get(&current_state)
                {
                    for &next_state in &next_states.empty {
                        self.current_states.insert(next_state);
                    }
                }
            }

            if prev_state_count == self.current_states.len() {
                break;
            }
            prev_state_count = self.current_states.len();
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::{Automaton, State, TransitionOutput};
    use std::collections::{BTreeSet, HashMap, HashSet};

    pub fn all_ones_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: State(0),
            final_states: HashSet::from([State(1)]),
            transitions: HashMap::from([(State(0), TransitionOutput {
                empty: BTreeSet::from([State(1)]),
                symbols: HashMap::from([(true, BTreeSet::from([State(0)]))]),
            })]),
        }
    }

    pub fn one_alternation_or_be_odd_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: State(0),
            final_states: HashSet::from([State(1), State(2), State(6)]),
            transitions: HashMap::from([
                (State(0), TransitionOutput {
                    empty: BTreeSet::from([State(1), State(2)]),
                    symbols: HashMap::from([
                        (true, BTreeSet::from([State(3)])),
                        (false, BTreeSet::from([State(4)])),
                    ]),
                }),
                (State(1), TransitionOutput {
                    empty: BTreeSet::from([State(1)]),
                    symbols: HashMap::from([(
                        true,
                        BTreeSet::from([State(1)]),
                    )]),
                }),
                (State(2), TransitionOutput {
                    empty: BTreeSet::from([State(2)]),
                    symbols: HashMap::from([(
                        false,
                        BTreeSet::from([State(2), State(5)]),
                    )]),
                }),
                (State(3), TransitionOutput {
                    empty: BTreeSet::from([State(1)]),
                    symbols: HashMap::from([
                        (true, BTreeSet::from([State(3)])),
                        (false, BTreeSet::from([State(2)])),
                    ]),
                }),
                (State(4), TransitionOutput {
                    empty: BTreeSet::from([State(2)]),
                    symbols: HashMap::from([
                        (true, BTreeSet::from([State(1)])),
                        (false, BTreeSet::from([State(4)])),
                    ]),
                }),
                (State(5), TransitionOutput {
                    empty: BTreeSet::from([]),
                    symbols: HashMap::from([
                        (true, BTreeSet::from([State(5), State(6)])),
                        (false, BTreeSet::from([State(5)])),
                    ]),
                }),
                (State(6), TransitionOutput {
                    empty: BTreeSet::from([]),
                    symbols: HashMap::from([]),
                }),
            ]),
        }
    }

    #[test]
    fn all_ones() {
        let automaton = all_ones_automaton();
        assert!(automaton.test(&[]));
        assert!(!automaton.test(&[false]));
        assert!(automaton.test(&[true]));
        assert!(!automaton.test(&[false, false]));
        assert!(!automaton.test(&[false, true]));
        assert!(!automaton.test(&[true, false]));
        assert!(automaton.test(&[true, true]));
        assert!(!automaton.test(&[false, true, false]));
        assert!(automaton.test(&[true, true, true]));
        assert!(!automaton.test(&[false, false, true]));
        assert!(!automaton.test(&[true, true, true, false]));
        assert!(!automaton.test(&[false, true, false, true]));
    }

    #[test]
    fn one_alternation_or_be_odd() {
        let automaton = one_alternation_or_be_odd_automaton();
        assert!(automaton.test(&[]));
        assert!(automaton.test(&[false]));
        assert!(automaton.test(&[true]));
        assert!(automaton.test(&[true, true]));
        assert!(automaton.test(&[true, true, true]));
        assert!(automaton.test(&[true, true, false]));
        assert!(automaton.test(&[true, true, false, false, false]));
        assert!(!automaton.test(&[true, false, true]));
        assert!(!automaton.test(&[false, false, true, false]));
        assert!(!automaton.test(&[false, false, true, true, false, false]));
        assert!(automaton.test(&[false, true, false, true, false, true]));
    }
}
