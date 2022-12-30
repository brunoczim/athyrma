use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

pub type State = u128;

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
    pub fn maximum_state(&self) -> State {
        let max_final_state = self.final_states.iter().copied().max();
        let max_table_state = self
            .transitions
            .iter()
            .map(|(state_in, states_out)| {
                let max_state_out =
                    states_out.values().flatten().copied().max();
                (*state_in).max(max_state_out.unwrap_or(State::MIN))
            })
            .max();
        self.initial_state
            .max(max_final_state.unwrap_or(State::MIN))
            .max(max_table_state.unwrap_or(State::MIN))
    }

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
    use super::Automaton;
    use std::collections::{BTreeSet, HashMap, HashSet};

    pub fn big_endian_binary_odd_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: 0,
            final_states: HashSet::from([1]),
            transitions: HashMap::from([(
                0,
                HashMap::from([
                    (false, BTreeSet::from([0])),
                    (true, BTreeSet::from([0, 1])),
                ]),
            )]),
        }
    }

    pub fn palindrome_4bit_automaton() -> Automaton<bool> {
        Automaton {
            initial_state: 0,
            final_states: HashSet::from([4, 10]),
            transitions: HashMap::from([
                (
                    0,
                    HashMap::from([
                        (false, BTreeSet::from([1, 5])),
                        (true, BTreeSet::from([7, 11])),
                    ]),
                ),
                (1, HashMap::from([(false, BTreeSet::from([2]))])),
                (2, HashMap::from([(false, BTreeSet::from([3]))])),
                (3, HashMap::from([(false, BTreeSet::from([4]))])),
                (5, HashMap::from([(true, BTreeSet::from([6]))])),
                (6, HashMap::from([(true, BTreeSet::from([3]))])),
                (7, HashMap::from([(false, BTreeSet::from([8]))])),
                (8, HashMap::from([(false, BTreeSet::from([9]))])),
                (9, HashMap::from([(true, BTreeSet::from([10]))])),
                (11, HashMap::from([(true, BTreeSet::from([12]))])),
                (12, HashMap::from([(true, BTreeSet::from([9]))])),
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
