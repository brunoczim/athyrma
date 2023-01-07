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

    pub fn merge(automatons: &[Self]) -> Self
    where
        T: Clone,
    {
        let initial_state = State(0);
        let mut final_states = HashSet::new();
        let mut transitions = HashMap::<_, HashMap<_, BTreeSet<_>>>::new();

        let mut state_count = 1;
        let mut state_mapping = vec![HashMap::new(); automatons.len()];
        let mut make_state = || {
            let new_state = State(state_count);
            state_count += 1;
            new_state
        };

        for (i, automaton) in automatons.iter().enumerate() {
            state_mapping[i].insert(automaton.initial_state, initial_state);

            for final_state in &automaton.final_states {
                let new_final_state = *state_mapping[i]
                    .entry(*final_state)
                    .or_insert_with(&mut make_state);
                final_states.insert(new_final_state);
            }

            for (current_state, next_states) in &automaton.transitions {
                let new_current_state = *state_mapping[i]
                    .entry(*current_state)
                    .or_insert_with(&mut make_state);

                let new_next_states =
                    transitions.entry(new_current_state).or_default();

                for (symbol, next_for_symbol) in next_states {
                    let new_next_for_symbol =
                        new_next_states.entry(symbol.clone()).or_default();
                    for next_state in next_for_symbol {
                        let new_next_state = *state_mapping[i]
                            .entry(*next_state)
                            .or_insert_with(&mut make_state);
                        new_next_for_symbol.insert(new_next_state);
                    }
                }
            }
        }

        Self { initial_state, final_states, transitions }
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

    pub fn mreged_binary_odd_and_palindrome_automaton() -> Automaton<bool> {
        Automaton::merge(&[
            big_endian_binary_odd_automaton(),
            palindrome_4bit_automaton(),
        ])
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

    #[test]
    fn merge() {
        let automaton = mreged_binary_odd_and_palindrome_automaton();
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
        assert!(!automaton.test(&[false]));
        assert!(!automaton.test(&[true, true, true, false]));
        assert!(!automaton.test(&[false, false, true, false]));
        assert!(!automaton.test(&[true, false, true, false]));
        assert!(automaton.test(&[false, true, true, false]));
        assert!(automaton.test(&[false, false, false, false]));
        assert!(automaton.test(&[true, true, true, true]));
        assert!(automaton.test(&[true, false, false, true]));
        // This now works due to the merging
        // assert!(!automaton.test(&[false, false, true]));
        // assert!(!automaton.test(&[false, true]));
        // assert!(!automaton.test(&[false, false, true, true]));
    }
}
