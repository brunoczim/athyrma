use crate::{dfa, em_nfa, nfa};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

pub fn nfa_to_dfa<T>(input: &nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone + std::fmt::Debug,
{
    let mut debug_i = 0;

    let mut iteration = NfaToDfaIteration::new(input);
    iteration.process_input();
    let (mut converged, mut nfa_automaton) = iteration.finish();
    while !converged {
        let mut iteration = NfaToDfaIteration::new(&nfa_automaton);
        iteration.process_input();
        let (iter_converged, new_automaton) = iteration.finish();
        converged = iter_converged;
        nfa_automaton = new_automaton;

        if debug_i == 32 || debug_i == 33 {
            dbg!(&nfa_automaton);
        }

        debug_i += 1;
    }
    let mut translater = NfaToDfaTranslator::new(&nfa_automaton);
    translater.translate()
}

pub fn em_nfa_to_dfa<T>(input: &em_nfa::Automaton<T>) -> dfa::Automaton<T>
where
    T: Hash + Ord + Clone + std::fmt::Debug,
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
struct NfaToDfaIteration<'input, T>
where
    T: Hash + Ord + Clone,
{
    nfa_automaton: &'input nfa::Automaton<T>,
    state_count: nfa::State,
    input_set_to_output: HashMap<&'input BTreeSet<nfa::State>, nfa::State>,
    input_to_output_set: HashMap<nfa::State, BTreeSet<nfa::State>>,
}

impl<'input, T> NfaToDfaIteration<'input, T>
where
    T: Hash + Ord + Clone,
{
    pub(self) fn new(nfa_automaton: &'input nfa::Automaton<T>) -> Self {
        Self {
            nfa_automaton,
            state_count: nfa::State(1),
            input_set_to_output: HashMap::new(),
            input_to_output_set: HashMap::from([(
                nfa_automaton.initial_state,
                BTreeSet::from([nfa::State(0)]),
            )]),
        }
    }

    pub(self) fn process_input(&mut self) {
        self.map_states();
    }

    pub(self) fn finish(&self) -> (bool, nfa::Automaton<T>) {
        let (converged, output_transitions) = self.output_transitions();
        let output_final_states = self.output_final_states();
        (converged, nfa::Automaton {
            initial_state: nfa::State(0),
            transitions: output_transitions,
            final_states: output_final_states,
        })
    }

    fn map_states(&mut self) {
        for (_, next_states) in &self.nfa_automaton.transitions {
            for (_, next_for_symbol) in next_states {
                self.input_set_to_output(next_for_symbol);
            }
        }
    }

    fn input_set_to_output(
        &mut self,
        input_set: &'input BTreeSet<nfa::State>,
    ) -> nfa::State {
        let output_state =
            *self.input_set_to_output.entry(input_set).or_insert_with(|| {
                let new_state = self.state_count;
                self.state_count.0 += 1;
                new_state
            });

        for input_state in input_set {
            self.input_to_output_set
                .entry(*input_state)
                .or_default()
                .insert(output_state);
        }

        output_state
    }

    fn output_transitions(
        &self,
    ) -> (bool, HashMap<nfa::State, HashMap<T, BTreeSet<nfa::State>>>) {
        let mut converged = true;
        let mut transitions = HashMap::<_, HashMap<T, BTreeSet<_>>>::new();

        for (curr_state, next_states) in &self.nfa_automaton.transitions {
            if let Some(new_currs) = self.input_to_output_set.get(curr_state) {
                for (symbol, next_for_symbol) in next_states {
                    let new_next = self
                        .input_set_to_output
                        .get(next_for_symbol)
                        .expect("NFA set state was previoussly mapped");
                    for new_curr in new_currs {
                        let new_next_states = transitions
                            .entry(*new_curr)
                            .or_default()
                            .entry(symbol.clone())
                            .or_default();
                        new_next_states.insert(*new_next);
                        if new_next_states.len() > 1 {
                            converged = false;
                        }
                    }
                }
            }
        }

        (converged, transitions)
    }

    fn output_final_states(&self) -> HashSet<nfa::State> {
        let mut final_states = HashSet::new();
        for final_state in &self.nfa_automaton.final_states {
            if let Some(states) = self.input_to_output_set.get(final_state) {
                final_states.extend(states.iter().copied());
            }
        }
        final_states
    }
}

#[derive(Debug)]
struct NfaToDfaTranslator<'input, T>
where
    T: Hash + Ord + Clone,
{
    nfa_automaton: &'input nfa::Automaton<T>,
    state_count: dfa::State,
    state_map: HashMap<nfa::State, dfa::State>,
}

impl<'input, T> NfaToDfaTranslator<'input, T>
where
    T: Hash + Ord + Clone,
{
    pub(self) fn new(nfa_automaton: &'input nfa::Automaton<T>) -> Self {
        Self {
            nfa_automaton,
            state_count: dfa::State(0),
            state_map: HashMap::new(),
        }
    }

    fn map_state(&mut self, state: nfa::State) -> dfa::State {
        *self.state_map.entry(state).or_insert_with(|| {
            let new_state = self.state_count;
            self.state_count.0 += 1;
            new_state
        })
    }

    pub(self) fn translate(&mut self) -> dfa::Automaton<T> {
        let nfa_automaton = &*self.nfa_automaton;
        let initial_state = self.map_state(nfa_automaton.initial_state);
        let final_states = nfa_automaton
            .final_states
            .iter()
            .copied()
            .map(|state| self.map_state(state))
            .collect();
        let mut transitions = HashMap::<_, HashMap<T, _>>::new();

        for (current, next_states) in &nfa_automaton.transitions {
            for (symbol, next_for_symbol) in next_states {
                if let Some(next) = next_for_symbol.first() {
                    let old = transitions
                        .entry(self.map_state(*current))
                        .or_default()
                        .insert(symbol.clone(), self.map_state(*next));
                    debug_assert!(old.is_none());
                }
            }
        }

        dfa::Automaton { initial_state, final_states, transitions }
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
