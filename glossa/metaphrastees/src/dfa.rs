use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

pub type State = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnrecognizedInput;

impl fmt::Display for UnrecognizedInput {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("input not recognized by the automaton")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Automaton<T>
where
    T: Hash + Eq,
{
    pub initial_state: State,
    pub final_states: HashSet<State>,
    pub transitions: HashMap<State, HashMap<T, State>>,
}

impl<T> Automaton<T>
where
    T: Hash + Eq,
{
    pub fn maximum_state(&self) -> State {
        let max_final_state = self.final_states.iter().copied().max();
        let max_table_state = self
            .transitions
            .iter()
            .map(|(&state_in, states_out)| {
                let max_state_out = states_out.values().copied().max();
                state_in.max(max_state_out.unwrap_or(State::MIN))
            })
            .max();
        self.initial_state
            .max(max_final_state.unwrap_or(State::MIN))
            .max(max_table_state.unwrap_or(State::MIN))
    }

    pub fn start(&self) -> Execution<T> {
        Execution { automaton: self, current_state: Ok(self.initial_state) }
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
        match execution.current_state() {
            Ok(state) => self.final_states.contains(&state),
            Err(_) => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Execution<'automaton, T>
where
    T: Hash + Eq,
{
    automaton: &'automaton Automaton<T>,
    current_state: Result<State, UnrecognizedInput>,
}

impl<'automaton, T> Clone for Execution<'automaton, T>
where
    T: Hash + Eq,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'automaton, T> Copy for Execution<'automaton, T> where T: Hash + Eq {}

impl<'automaton, T> Execution<'automaton, T>
where
    T: Hash + Eq,
{
    pub fn current_state(&self) -> Result<State, UnrecognizedInput> {
        self.current_state
    }

    pub fn next(&mut self, symbol: &T) {
        if let Ok(current_state) = self.current_state {
            self.current_state = self
                .automaton
                .transitions
                .get(&current_state)
                .and_then(|transitions| transitions.get(symbol))
                .copied()
                .ok_or(UnrecognizedInput);
        }
    }
}
