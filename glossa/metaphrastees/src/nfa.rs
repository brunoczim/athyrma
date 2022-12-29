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
        !execution.current_states().is_empty()
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
