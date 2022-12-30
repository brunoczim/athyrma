use crate::{dfa, nfa};
use std::{
    collections::{hash_map, BTreeSet, HashMap, HashSet},
    fmt,
    hash::Hash,
};

pub fn nfa_to_dfa<T>(input: &nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone + fmt::Debug,
{
    let mut compiler = NfaToDfa::new(input.initial_state);
    compiler.process_nfa_transitions(&input.transitions);
    let final_states = compiler.dfa_final_states(&input.final_states);
    let transitions = compiler.dfa_transitions();
    dfa::Automaton { initial_state: dfa::State(0), final_states, transitions }
}

#[derive(Debug)]
struct NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone + fmt::Debug,
{
    state_count: dfa::State,
    nfa_set_to_dfa: HashMap<&'input BTreeSet<nfa::State>, dfa::State>,
    nfa_to_dfa_set: HashMap<nfa::State, BTreeSet<dfa::State>>,
    transitions: HashMap<nfa::State, HashMap<&'input T, BTreeSet<dfa::State>>>,
}

impl<'input, T> NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone + fmt::Debug,
{
    fn new(initial_nfa_state: nfa::State) -> Self {
        Self {
            state_count: dfa::State(1),
            nfa_set_to_dfa: HashMap::new(),
            nfa_to_dfa_set: HashMap::from([(
                initial_nfa_state,
                BTreeSet::from([dfa::State(0)]),
            )]),
            transitions: HashMap::new(),
        }
    }

    fn nfa_set_to_dfa(
        &mut self,
        nfa_set: &'input BTreeSet<nfa::State>,
    ) -> dfa::State {
        let dfa_state =
            *self.nfa_set_to_dfa.entry(nfa_set).or_insert_with(|| {
                let new_state = self.state_count;
                self.state_count.0 += 1;
                new_state
            });

        for nfa_state in nfa_set {
            self.nfa_to_dfa_set
                .entry(*nfa_state)
                .or_default()
                .insert(dfa_state);
        }

        dfa_state
    }

    pub(self) fn process_nfa_transitions(
        &mut self,
        transitions: &'input HashMap<
            nfa::State,
            HashMap<T, BTreeSet<nfa::State>>,
        >,
    ) {
        for (current_state, next_states) in transitions {
            self.process_nfa_transition_entry(*current_state, next_states);
        }
    }

    fn process_nfa_transition_entry(
        &mut self,
        current_state: nfa::State,
        next_states: &'input HashMap<T, BTreeSet<nfa::State>>,
    ) {
        let mut mapped_next_states = HashMap::<_, BTreeSet<_>>::new();
        for (symbol, next_for_symbol) in next_states {
            let dfa_state = self.nfa_set_to_dfa(next_for_symbol);
            mapped_next_states.entry(symbol).or_default().insert(dfa_state);
        }

        match self.transitions.entry(current_state) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().extend(mapped_next_states);
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert(mapped_next_states);
            },
        };
    }

    fn dfa_final_states(
        &self,
        nfa_final_states: &HashSet<nfa::State>,
    ) -> HashSet<dfa::State> {
        let mut final_states = HashSet::new();
        for final_state in nfa_final_states {
            if let Some(states) = self.nfa_to_dfa_set.get(final_state) {
                final_states.extend(states.iter().copied());
            }
        }
        final_states
    }

    fn dfa_transitions(&self) -> HashMap<dfa::State, HashMap<T, dfa::State>> {
        let mut dfa_transitions = HashMap::<_, HashMap<T, _>>::new();

        for (current_state, next_states) in &self.transitions {
            let mut dfa_next_states = HashMap::new();
            for (symbol, next_for_symbol) in next_states {
                for dfa_state in next_for_symbol {
                    dfa_next_states.insert(symbol.clone(), dfa_state);
                }
            }

            let dfa_states = self.nfa_to_dfa_set.get(&current_state).unwrap();
            for dfa_state in dfa_states {
                dfa_transitions.entry(*dfa_state).or_default().extend(
                    dfa_next_states.iter().map(|(key, value)| {
                        ((**key).clone(), (**value).clone())
                    }),
                );
            }
        }

        dfa_transitions
    }
}

#[cfg(test)]
mod test {
    use crate::{
        compiler::nfa_to_dfa,
        nfa::test::{
            big_endian_binary_odd_automaton,
            palindrome_4bit_automaton,
        },
    };

    #[test]
    fn binary_odd() {
        let nfa_automaton = big_endian_binary_odd_automaton();
        let dfa_automaton = nfa_to_dfa(&nfa_automaton);
        assert!(!dfa_automaton.test(&[]));
        assert!(!dfa_automaton.test(&[false]));
        assert!(dfa_automaton.test(&[true]));
        assert!(!dfa_automaton.test(&[false, false]));
        assert!(dfa_automaton.test(&[false, true]));
        assert!(!dfa_automaton.test(&[true, false]));
        assert!(dfa_automaton.test(&[true, true]));
        assert!(!dfa_automaton.test(&[false, true, false]));
        assert!(dfa_automaton.test(&[false, false, true]));
        assert!(!dfa_automaton.test(&[true, true, true, false]));
        assert!(dfa_automaton.test(&[false, true, false, true]));
    }

    #[test]
    fn palindrome_4bit() {
        let nfa_automaton = palindrome_4bit_automaton();
        let dfa_automaton = nfa_to_dfa(&nfa_automaton);
        assert!(!dfa_automaton.test(&[]));
        assert!(!dfa_automaton.test(&[false]));
        assert!(!dfa_automaton.test(&[false, true]));
        assert!(!dfa_automaton.test(&[false, false, true]));
        assert!(!dfa_automaton.test(&[true, true, true, false]));
        assert!(!dfa_automaton.test(&[false, false, true, true]));
        assert!(!dfa_automaton.test(&[false, false, true, false]));
        assert!(!dfa_automaton.test(&[true, false, true, false]));
        assert!(dfa_automaton.test(&[false, true, true, false]));
        assert!(dfa_automaton.test(&[false, false, false, false]));
        assert!(dfa_automaton.test(&[true, true, true, true]));
        assert!(dfa_automaton.test(&[true, false, false, true]));
    }
}
