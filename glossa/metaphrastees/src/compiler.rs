use crate::{dfa, em_nfa, nfa};
use std::{
    collections::{hash_map, BTreeSet, HashMap, HashSet},
    hash::Hash,
};

pub fn nfa_to_dfa<T>(input: &nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone,
{
    let mut compiler = NfaToDfa::new(input.initial_state);
    compiler.process_nfa_transitions(&input.transitions);
    let final_states = compiler.dfa_final_states(&input.final_states);
    let transitions = compiler.dfa_transitions();
    dfa::Automaton { initial_state: dfa::State(0), final_states, transitions }
}

pub fn em_nfa_to_dfa<T>(input: &em_nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone,
{
    let nfa_automaton = em_nfa_to_nfa(input);
    nfa_to_dfa(&nfa_automaton)
}

pub fn em_nfa_to_nfa<T>(input: &em_nfa::Automaton<T>) -> nfa::Automaton<T>
where
    T: Hash + Ord + Clone,
{
    let mut compiler = EmNfaToNfa::new(input.initial_state);
    compiler
        .process_em_nfa_transitions(&input.transitions, &input.final_states);
    compiler.process_em_nfa_final_states(&input.final_states);
    compiler.nfa_automaton(input.initial_state)
}

pub fn nfa_to_em_nfa<T>(input: &nfa::Automaton<T>) -> em_nfa::Automaton<T>
where
    T: Hash + Ord + Clone,
{
    em_nfa::Automaton {
        initial_state: em_nfa::State(input.initial_state.0),
        final_states: input
            .final_states
            .iter()
            .map(|state| em_nfa::State(state.0))
            .collect(),
        transitions: input
            .transitions
            .iter()
            .map(|(state, next_states)| {
                (em_nfa::State(state.0), em_nfa::TransitionOutput {
                    empty: BTreeSet::new(),
                    symbols: next_states
                        .iter()
                        .map(|(symbol, next_for_symbol)| {
                            (
                                symbol.clone(),
                                next_for_symbol
                                    .iter()
                                    .map(|state| em_nfa::State(state.0))
                                    .collect(),
                            )
                        })
                        .collect(),
                })
            })
            .collect(),
    }
}

#[derive(Debug)]
struct NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone,
{
    state_count: dfa::State,
    nfa_set_to_dfa: HashMap<&'input BTreeSet<nfa::State>, dfa::State>,
    nfa_to_dfa_set: HashMap<nfa::State, BTreeSet<dfa::State>>,
    transitions: HashMap<nfa::State, HashMap<&'input T, BTreeSet<dfa::State>>>,
}

impl<'input, T> NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone,
{
    pub(self) fn new(initial_nfa_state: nfa::State) -> Self {
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
        }
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

            let dfa_states = self.nfa_to_dfa_set.get(&current_state).expect(
                "handle nfa transitions must be called before dfa transitions",
            );
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

#[derive(Debug)]
struct EmNfaToNfa<T>
where
    T: Hash + Ord + Clone,
{
    state_count: nfa::State,
    em_nfa_to_nfa: HashMap<em_nfa::State, nfa::State>,
    final_states: HashSet<nfa::State>,
    transitions: HashMap<nfa::State, HashMap<T, BTreeSet<nfa::State>>>,
}

impl<T> EmNfaToNfa<T>
where
    T: Hash + Ord + Clone,
{
    pub(self) fn new(em_nfa_initial_state: em_nfa::State) -> Self {
        let mut this = Self {
            state_count: nfa::State(0),
            em_nfa_to_nfa: HashMap::new(),
            final_states: HashSet::new(),
            transitions: HashMap::new(),
        };
        this.em_nfa_to_nfa(em_nfa_initial_state);
        this
    }

    fn em_nfa_to_nfa(&mut self, em_nfa: em_nfa::State) -> nfa::State {
        *self.em_nfa_to_nfa.entry(em_nfa).or_insert_with(|| {
            let new_state = self.state_count;
            self.state_count.0 += 1;
            new_state
        })
    }

    fn process_em_nfa_final_states(
        &mut self,
        final_states: &HashSet<em_nfa::State>,
    ) {
        for final_state in final_states {
            let new_final_state = self.em_nfa_to_nfa(*final_state);
            self.final_states.insert(new_final_state);
        }
    }

    fn process_em_nfa_transitions(
        &mut self,
        transitions: &HashMap<em_nfa::State, em_nfa::TransitionOutput<T>>,
        final_states: &HashSet<em_nfa::State>,
    ) {
        for (current_state, next_states) in transitions {
            self.process_em_nfa_symbol_transition_entry(
                *current_state, &next_states.symbols,
            );
            self.process_em_nfa_em_transition_entry(
                *current_state, &next_states.empty, transitions, final_states,
            );
        }
    }

    fn process_em_nfa_symbol_transition_entry(
        &mut self,
        current_state: em_nfa::State,
        next_states: &HashMap<T, BTreeSet<em_nfa::State>>,
    ) {
        let new_current_state = self.em_nfa_to_nfa(current_state);
        for (symbol, next_for_symbol) in next_states {
            for &next_state in next_for_symbol {
                let new_next_state = self.em_nfa_to_nfa(next_state);
                self.transitions
                    .entry(new_current_state)
                    .or_default()
                    .entry(symbol.clone())
                    .or_default()
                    .insert(new_next_state);
            }
        }
    }

    fn process_em_nfa_em_transition_entry(
        &mut self,
        current_state: em_nfa::State,
        empty_moves: &BTreeSet<em_nfa::State>,
        transitions: &HashMap<em_nfa::State, em_nfa::TransitionOutput<T>>,
        final_states: &HashSet<em_nfa::State>,
    ) {
        let mut visited = HashSet::new();
        let mut states = empty_moves.clone();
        let new_current_state = self.em_nfa_to_nfa(current_state);

        while let Some(state) = states.pop_first() {
            if visited.insert(state) {
                if let Some(entry) = transitions.get(&state) {
                    self.process_em_nfa_symbol_transition_entry(
                        current_state, &entry.symbols,
                    );
                    states.extend(entry.empty.iter().copied());
                }

                if final_states.contains(&state) {
                    self.final_states.insert(new_current_state);
                }
            }
        }
    }

    fn nfa_automaton(
        mut self,
        em_nfa_initial_state: em_nfa::State,
    ) -> nfa::Automaton<T> {
        nfa::Automaton {
            initial_state: self.em_nfa_to_nfa(em_nfa_initial_state),
            final_states: self.final_states,
            transitions: self.transitions,
        }
    }
}

#[cfg(test)]
mod test {
    use super::em_nfa_to_nfa;
    use crate::{
        compiler::{em_nfa_to_dfa, nfa_to_dfa},
        em_nfa::test::{
            all_ones_automaton,
            one_alternation_or_be_odd_automaton,
        },
        nfa::test::{
            big_endian_binary_odd_automaton,
            palindrome_4bit_automaton,
        },
    };

    #[test]
    fn nfa_to_dfa_binary_odd() {
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
    fn nfa_to_dfa_palindrome_4bit() {
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

    #[test]
    fn em_nfa_to_nfa_all_ones() {
        let em_nfa_automaton = all_ones_automaton();
        let nfa_automaton = em_nfa_to_nfa(&em_nfa_automaton);
        assert!(nfa_automaton.test(&[]));
        assert!(!nfa_automaton.test(&[false]));
        assert!(nfa_automaton.test(&[true]));
        assert!(!nfa_automaton.test(&[false, false]));
        assert!(!nfa_automaton.test(&[false, true]));
        assert!(!nfa_automaton.test(&[true, false]));
        assert!(nfa_automaton.test(&[true, true]));
        assert!(!nfa_automaton.test(&[false, true, false]));
        assert!(nfa_automaton.test(&[true, true, true]));
        assert!(!nfa_automaton.test(&[false, false, true]));
        assert!(!nfa_automaton.test(&[true, true, true, false]));
        assert!(!nfa_automaton.test(&[false, true, false, true]));
    }

    #[test]
    fn em_nfa_to_nfa_one_alter_or_be_odd() {
        let em_nfa_automaton = one_alternation_or_be_odd_automaton();
        let nfa_automaton = em_nfa_to_nfa(&em_nfa_automaton);
        assert!(nfa_automaton.test(&[]));
        assert!(nfa_automaton.test(&[false]));
        assert!(nfa_automaton.test(&[true]));
        assert!(nfa_automaton.test(&[true, true]));
        assert!(nfa_automaton.test(&[true, true, true]));
        assert!(nfa_automaton.test(&[true, true, false]));
        assert!(nfa_automaton.test(&[true, true, false, false, false]));
        assert!(!nfa_automaton.test(&[true, false, true]));
        assert!(!nfa_automaton.test(&[false, false, true, false]));
        assert!(!nfa_automaton.test(&[false, false, true, true, false, false]));
        assert!(nfa_automaton.test(&[false, true, false, true, false, true]));
    }

    #[test]
    fn em_nfa_to_dfa_all_ones() {
        let em_dfa_automaton = all_ones_automaton();
        let dfa_automaton = em_nfa_to_dfa(&em_dfa_automaton);
        assert!(dfa_automaton.test(&[]));
        assert!(!dfa_automaton.test(&[false]));
        assert!(dfa_automaton.test(&[true]));
        assert!(!dfa_automaton.test(&[false, false]));
        assert!(!dfa_automaton.test(&[false, true]));
        assert!(!dfa_automaton.test(&[true, false]));
        assert!(dfa_automaton.test(&[true, true]));
        assert!(!dfa_automaton.test(&[false, true, false]));
        assert!(dfa_automaton.test(&[true, true, true]));
        assert!(!dfa_automaton.test(&[false, false, true]));
        assert!(!dfa_automaton.test(&[true, true, true, false]));
        assert!(!dfa_automaton.test(&[false, true, false, true]));
    }

    #[test]
    fn em_nfa_to_dfa_one_alter_or_be_odd() {
        let em_dfa_automaton = one_alternation_or_be_odd_automaton();
        let dfa_automaton = em_nfa_to_dfa(&em_dfa_automaton);
        assert!(dfa_automaton.test(&[]));
        assert!(dfa_automaton.test(&[false]));
        assert!(dfa_automaton.test(&[true]));
        assert!(dfa_automaton.test(&[true, true]));
        assert!(dfa_automaton.test(&[true, true, true]));
        assert!(dfa_automaton.test(&[true, true, false]));
        assert!(dfa_automaton.test(&[true, true, false, false, false]));
        assert!(!dfa_automaton.test(&[true, false, true]));
        assert!(!dfa_automaton.test(&[false, false, true, false]));
        assert!(!dfa_automaton.test(&[false, false, true, true, false, false]));
        assert!(dfa_automaton.test(&[false, true, false, true, false, true]));
    }
}
