use crate::{dfa, nfa};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

pub fn nfa_to_dfa<T>(input: &nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone,
{
    let mut compiler = NfaToDfa::new(input.initial_state);
    compiler.handle_transitions(&input.transitions);
    let final_states = compiler.final_states(&input.final_states);
    let transitions = compiler.into_transitions();
    dfa::Automaton { initial_state: 0, final_states, transitions }
}

#[derive(Debug)]
struct NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone,
{
    state_count: dfa::State,
    nfa_set_to_dfa: HashMap<&'input BTreeSet<nfa::State>, dfa::State>,
    nfa_to_dfa_set: HashMap<nfa::State, BTreeSet<dfa::State>>,
    dfa_transitions: HashMap<dfa::State, HashMap<T, dfa::State>>,
}

impl<'input, T> NfaToDfa<'input, T>
where
    T: Hash + Ord + Clone,
{
    fn new(initial_nfa_state: nfa::State) -> Self {
        Self {
            state_count: 1,
            nfa_set_to_dfa: HashMap::new(),
            nfa_to_dfa_set: HashMap::from([(
                initial_nfa_state,
                BTreeSet::from([0]),
            )]),
            dfa_transitions: HashMap::new(),
        }
    }

    fn nfa_set_to_dfa(
        &mut self,
        nfa_set: &'input BTreeSet<nfa::State>,
    ) -> dfa::State {
        let state = *self.nfa_set_to_dfa.entry(nfa_set).or_insert_with(|| {
            let new_state = self.state_count;
            self.state_count += 1;
            new_state
        });

        self.nfa_to_dfa_set
            .entry(state)
            .or_default()
            .extend(nfa_set.iter().copied());

        state
    }

    fn handle_transitions(
        &mut self,
        transitions: &'input HashMap<
            nfa::State,
            HashMap<T, BTreeSet<nfa::State>>,
        >,
    ) {
        for (current_state, next_states) in transitions {
            self.handle_transition_entry(*current_state, next_states);
        }
    }

    fn handle_transition_entry(
        &mut self,
        current_state: nfa::State,
        next_states: &'input HashMap<T, BTreeSet<nfa::State>>,
    ) {
        let mut new_next_states = HashMap::new();
        for (symbol, next_for_symbol) in next_states {
            let new_state = self.nfa_set_to_dfa(next_for_symbol);
            new_next_states.insert(symbol.clone(), new_state);
        }

        if let Some(dfa_states) = self.nfa_to_dfa_set.get(&current_state) {
            for dfa_state in dfa_states {
                self.dfa_transitions
                    .insert(*dfa_state, new_next_states.clone());
            }
        }
    }

    fn final_states(
        &self,
        nfa_final_states: &HashSet<dfa::State>,
    ) -> HashSet<dfa::State> {
        let mut final_states = HashSet::new();
        for final_state in nfa_final_states {
            if let Some(states) = self.nfa_to_dfa_set.get(final_state) {
                final_states.extend(states.iter().copied());
            }
        }
        final_states
    }

    fn into_transitions(self) -> HashMap<dfa::State, HashMap<T, dfa::State>> {
        self.dfa_transitions
    }
}
