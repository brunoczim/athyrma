use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub type State = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub table: HashMap<(State, T), State>,
}

impl<T> Automaton<T>
where
    T: Hash + Eq,
{
    pub fn start(&self) -> Execution<T> {
        Execution { automaton: self, current_state: Ok(self.initial_state) }
    }

    pub fn test<I>(&self, input: I) -> bool
    where
        I: IntoIterator<Item = T>,
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

    pub fn next(&mut self, symbol: T) {
        if let Ok(current_state) = self.current_state {
            self.current_state = self
                .automaton
                .table
                .get(&(current_state, symbol))
                .copied()
                .ok_or(UnrecognizedInput);
        }
    }
}
