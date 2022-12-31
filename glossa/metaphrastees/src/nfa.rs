use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(pub u128);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Automaton<T>
where
    T: Hash + Ord,
{
    pub initial_state: State,
    pub final_states: HashSet<State>,
    pub transitions: HashMap<State, HashMap<T, BTreeSet<State>>>,
}

impl<T> Automaton<T>
where
    T: Hash + Ord,
{
    pub fn start(&self) -> Execution<T> {
        Execution {
            automaton: self,
            current_states: HashSet::from([self.initial_state]),
        }
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
                if let Some(next_for_symbol) = next_states.get(symbol) {
                    for &next_state in next_for_symbol {
                        self.current_states.insert(next_state);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{Automaton, State};
    use std::collections::{BTreeSet, HashMap, HashSet};

    pub fn big_endian_binary_odd_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: State(0),
            final_states: HashSet::from([State(1)]),
            transitions: HashMap::from([(
                State(0),
                HashMap::from([
                    (false, BTreeSet::from([State(0)])),
                    (true, BTreeSet::from([State(0), State(1)])),
                ]),
            )]),
        }
    }

    pub fn palindrome_4bit_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: State(0),
            final_states: HashSet::from([State(4), State(10)]),
            transitions: HashMap::from([
                (
                    State(0),
                    HashMap::from([
                        (false, BTreeSet::from([State(1), State(5)])),
                        (true, BTreeSet::from([State(7), State(11)])),
                    ]),
                ),
                (
                    State(1),
                    HashMap::from([(false, BTreeSet::from([State(2)]))]),
                ),
                (
                    State(2),
                    HashMap::from([(false, BTreeSet::from([State(3)]))]),
                ),
                (
                    State(3),
                    HashMap::from([(false, BTreeSet::from([State(4)]))]),
                ),
                (State(5), HashMap::from([(true, BTreeSet::from([State(6)]))])),
                (State(6), HashMap::from([(true, BTreeSet::from([State(3)]))])),
                (
                    State(7),
                    HashMap::from([(false, BTreeSet::from([State(8)]))]),
                ),
                (
                    State(8),
                    HashMap::from([(false, BTreeSet::from([State(9)]))]),
                ),
                (
                    State(9),
                    HashMap::from([(true, BTreeSet::from([State(10)]))]),
                ),
                (
                    State(11),
                    HashMap::from([(true, BTreeSet::from([State(12)]))]),
                ),
                (
                    State(12),
                    HashMap::from([(true, BTreeSet::from([State(9)]))]),
                ),
            ]),
        }
    }

    #[test]
    fn binary_odd() {
        let automaton = big_endian_binary_odd_automaton();
        assert!(!automaton.test(&[]));
        assert!(!automaton.test(&[false]));
        assert!(automaton.test(&[true]));
        assert!(!automaton.test(&[false, false]));
        assert!(automaton.test(&[false, true]));
        assert!(!automaton.test(&[true, false]));
        assert!(automaton.test(&[true, true]));
        assert!(!automaton.test(&[false, true, false]));
        assert!(automaton.test(&[false, false, true]));
        assert!(!automaton.test(&[true, true, true, false]));
        assert!(automaton.test(&[false, true, false, true]));
    }

    #[test]
    fn palindrome_4bit() {
        let automaton = palindrome_4bit_automaton();
        assert!(!automaton.test(&[]));
        assert!(!automaton.test(&[false]));
        assert!(!automaton.test(&[false, true]));
        assert!(!automaton.test(&[false, false, true]));
        assert!(!automaton.test(&[true, true, true, false]));
        assert!(!automaton.test(&[false, false, true, true]));
        assert!(!automaton.test(&[false, false, true, false]));
        assert!(!automaton.test(&[true, false, true, false]));
        assert!(automaton.test(&[false, true, true, false]));
        assert!(automaton.test(&[false, false, false, false]));
        assert!(automaton.test(&[true, true, true, true]));
        assert!(automaton.test(&[true, false, false, true]));
    }
}
