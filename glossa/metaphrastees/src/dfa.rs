use crate::{finite_automaton::FiniteAutomaton, symbol::ToIndex};
use core::fmt;
use std::{
    collections::{hash_map, hash_set, HashMap, HashSet},
    error::Error,
    hash::Hash,
    marker::PhantomData,
};

#[cold]
#[inline(never)]
fn panic_build_error(error: BuildError) -> ! {
    panic!("{}", error)
}

#[derive(Debug, Clone)]
pub enum BuildError {
    InvalidState {
        state: State,
    },
    TransitionConflict {
        current_state: State,
        stored_next_state: State,
        requested_next_state: State,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidState { state } => {
                write!(formatter, "invalid state {}", state)
            },
            Self::TransitionConflict {
                current_state,
                stored_next_state,
                requested_next_state,
            } => {
                write!(
                    formatter,
                    "transition conflict, requested insertion of next state \
                     {} on entry with current state {} and next state {}",
                    requested_next_state, current_state, stored_next_state
                )
            },
        }
    }
}

impl Error for BuildError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(u128);

impl State {
    pub const INITIAL: Self = Self(0);
}

impl fmt::Display for State {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "q{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct StateTable {
    counter: usize,
    map: HashMap<State, usize>,
}

impl StateTable {
    fn new() -> Self {
        Self { counter: 1, map: HashMap::from([(State::INITIAL, 0)]) }
    }
}

impl StateTable {
    fn map(&mut self, state: State) -> usize {
        *self.map.entry(state).or_insert_with(|| {
            let state = self.counter;
            self.counter += 1;
            state
        })
    }
}

#[derive(Debug, Clone)]
pub struct Builder<T> {
    state_counter: State,
    transitions: HashMap<State, HashMap<T, State>>,
    final_states: HashSet<State>,
}

impl<T> Builder<T> {
    pub fn new() -> Self {
        Self {
            state_counter: State(1),
            transitions: HashMap::new(),
            final_states: HashSet::new(),
        }
    }

    pub fn gen_state(&mut self) -> State {
        let state = self.state_counter;
        self.state_counter.0 += 1;
        state
    }

    fn check_state(&self, state: State) -> Result<(), BuildError> {
        if state < self.state_counter {
            Ok(())
        } else {
            Err(BuildError::InvalidState { state })
        }
    }
}

impl<T> Builder<T>
where
    T: Hash + Eq,
{
    pub fn try_transition(
        &mut self,
        current_state: State,
        symbol: T,
        next_state: State,
    ) -> Result<&mut Self, BuildError> {
        self.check_state(current_state)?;
        self.check_state(next_state)?;
        match self.transitions.entry(current_state).or_default().entry(symbol) {
            hash_map::Entry::Occupied(entry) => {
                Err(BuildError::TransitionConflict {
                    current_state,
                    stored_next_state: *entry.get(),
                    requested_next_state: next_state,
                })?
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert(next_state);
            },
        }
        Ok(self)
    }

    pub fn transition(
        &mut self,
        current_state: State,
        symbol: T,
        next_state: State,
    ) -> &mut Self {
        match self.try_transition(current_state, symbol, next_state) {
            Ok(this) => this,
            Err(error) => panic_build_error(error),
        }
    }

    pub fn try_add_final_state(
        &mut self,
        state: State,
    ) -> Result<&mut Self, BuildError> {
        self.check_state(state)?;
        self.final_states.insert(state);
        Ok(self)
    }

    pub fn add_final_state(&mut self, state: State) -> &mut Self {
        match self.try_add_final_state(state) {
            Ok(this) => this,
            Err(error) => panic_build_error(error),
        }
    }

    pub fn get_transition(
        &self,
        current_state: State,
        symbol: &T,
    ) -> Result<State, BuildError> {
        self.transitions
            .get(&current_state)
            .and_then(|table| table.get(&symbol))
            .copied()
            .ok_or(BuildError::InvalidState { state: current_state })
    }

    pub fn is_final_state(&self, state: State) -> bool {
        self.final_states.contains(&state)
    }

    pub fn iter_transitions(&self) -> IterTransitions<T> {
        IterTransitions { outer: self.transitions.iter(), current: None }
    }

    pub fn iter_next_states(
        &self,
        current_state: State,
    ) -> Result<IterNextStates<T>, BuildError> {
        Ok(IterNextStates {
            inner: self
                .transitions
                .get(&current_state)
                .ok_or(BuildError::InvalidState { state: current_state })?
                .iter(),
        })
    }

    pub fn iter_final_states(&self) -> IterFinalStates {
        IterFinalStates { inner: self.final_states.iter() }
    }

    pub fn drop_useless_transitions(&mut self) -> ReachableStates {
        let mut reachable = HashSet::new();
        let mut branches = vec![State::INITIAL];

        while let Some(current_state) = branches.pop() {
            if reachable.insert(current_state) {
                if let Ok(iterator) = self.iter_next_states(current_state) {
                    for (_, next_state) in iterator {
                        branches.push(next_state);
                    }
                }
            }
        }

        self.transitions.drain_filter(|_, next_states| {
            next_states
                .drain_filter(|_, next_state| !reachable.contains(&next_state));
            next_states.len() == 0
        });

        self.final_states.drain_filter(|state| !reachable.contains(&state));

        ReachableStates { count: reachable.len(), inner: reachable.into_iter() }
    }

    pub fn build_small(mut self) -> SmallAutomaton<T> {
        self.drop_useless_transitions();

        let mut state_table = StateTable::new();
        state_table.map(State::INITIAL);

        let mut automaton = SmallAutomaton {
            transitions: HashMap::new(),
            final_states: HashSet::new(),
        };

        for (current_state, next_states) in self.transitions {
            let new_current = state_table.map(current_state);
            for (symbol, next_state) in next_states {
                let new_next = state_table.map(next_state);
                automaton
                    .transitions
                    .entry(new_current)
                    .or_default()
                    .insert(symbol, new_next);
            }
        }

        automaton.final_states.extend(
            self.final_states.iter().map(|state| state_table.map(*state)),
        );

        automaton
    }
}

impl<T> Builder<T>
where
    T: Hash + Eq + ToIndex,
{
    pub fn build_fast(mut self) -> FastAutomaton<T> {
        let reachable = self.drop_useless_transitions();
        let mut state_table = StateTable::new();

        let words = FastAutomaton::<T>::words(reachable.total_count());
        let mut automaton = FastAutomaton {
            transitions: vec![usize::MAX; words],
            _marker: PhantomData,
        };

        for state in reachable {
            let new_state = state_table.map(state);
            let index = FastAutomaton::<T>::entry_final_state(new_state);
            let is_final = self.final_states.contains(&state);
            automaton.transitions[index] = usize::from(is_final);
        }

        for (current_state, next_states) in self.transitions {
            let new_current = state_table.map(current_state);
            for (symbol, next_state) in next_states {
                let new_next = state_table.map(next_state);
                let index =
                    FastAutomaton::<T>::entry_next_state(new_current, &symbol);
                automaton.transitions[index] = usize::from(new_next);
            }
        }

        automaton
    }
}

#[derive(Debug)]
pub struct IterTransitions<'builder, T> {
    outer: hash_map::Iter<'builder, State, HashMap<T, State>>,
    current: Option<(State, hash_map::Iter<'builder, T, State>)>,
}

impl<'builder, T> Iterator for IterTransitions<'builder, T> {
    type Item = (State, &'builder T, State);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.current {
                Some((current_state, table)) => match table.next() {
                    Some((symbol, next_state)) => {
                        break Some((*current_state, symbol, *next_state))
                    },
                    None => self.current = None,
                },
                None => {
                    let (current_state, table) = self.outer.next()?;
                    self.current = Some((*current_state, table.iter()));
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct IterNextStates<'builder, T> {
    inner: hash_map::Iter<'builder, T, State>,
}

impl<'builder, T> Iterator for IterNextStates<'builder, T> {
    type Item = (&'builder T, State);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(symbol, next_state)| (symbol, *next_state))
    }
}

#[derive(Debug)]
pub struct IterFinalStates<'builder> {
    inner: hash_set::Iter<'builder, State>,
}

impl<'builder> Iterator for IterFinalStates<'builder> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}

#[derive(Debug)]
pub struct ReachableStates {
    count: usize,
    inner: hash_set::IntoIter<State>,
}

impl ReachableStates {
    pub fn total_count(&self) -> usize {
        self.count
    }
}

impl Iterator for ReachableStates {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastAutomaton<T> {
    transitions: Vec<usize>,
    _marker: PhantomData<T>,
}

impl<T> FastAutomaton<T>
where
    T: ToIndex,
{
    fn entry_size() -> usize {
        1 + T::COUNT
    }

    fn entry_start(index: usize) -> usize {
        Self::entry_size() * index
    }

    fn words(state_count: usize) -> usize {
        Self::entry_size() * state_count
    }

    fn entry_final_state(current_state: usize) -> usize {
        Self::entry_start(current_state)
    }

    fn entry_next_state(current_state: usize, symbol: &T) -> usize {
        Self::entry_start(current_state) + 1 + symbol.to_index()
    }
}

impl<T> FiniteAutomaton<T> for FastAutomaton<T>
where
    T: ToIndex,
{
    fn test<'item, I>(&self, input: I) -> bool
    where
        T: 'item,
        I: IntoIterator<Item = &'item T>,
    {
        let mut current_state = 0;
        let mut iterator = input.into_iter();

        while let Some(symbol) = iterator.next() {
            let index = Self::entry_next_state(current_state, symbol);
            let next_state = self.transitions[index];
            if next_state == usize::MAX {
                return false;
            }
            current_state = next_state;
        }

        let index = Self::entry_final_state(current_state);
        self.transitions[index] != 0
    }
}

#[derive(Debug, Clone)]
pub struct SmallAutomaton<T> {
    transitions: HashMap<usize, HashMap<T, usize>>,
    final_states: HashSet<usize>,
}

impl<T> PartialEq for SmallAutomaton<T>
where
    T: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.transitions == other.transitions
            && self.final_states == other.final_states
    }
}

impl<T> Eq for SmallAutomaton<T> where T: Hash + Eq {}

impl<T> FiniteAutomaton<T> for SmallAutomaton<T>
where
    T: Hash + Eq,
{
    fn test<'item, I>(&self, input: I) -> bool
    where
        T: 'item,
        I: IntoIterator<Item = &'item T>,
    {
        let mut current_state = 0;
        let mut iterator = input.into_iter();

        while let Some(symbol) = iterator.next() {
            match self
                .transitions
                .get(&current_state)
                .and_then(|transitions| transitions.get(symbol))
                .copied()
            {
                Some(state) => current_state = state,
                None => return false,
            };
        }

        self.final_states.contains(&current_state)
    }
}

#[cfg(test)]
mod test {
    use super::{Builder, State};
    use crate::{
        finite_automaton::FiniteAutomaton,
        symbol::{Countable, FromIndex, ToIndex},
    };

    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash,
    )]
    pub struct Succ;

    impl Countable for Succ {
        const COUNT: usize = 1;
    }

    impl ToIndex for Succ {
        fn to_index(&self) -> usize {
            0
        }
    }

    impl FromIndex for Succ {
        fn from_index(_index: usize) -> Self {
            Self
        }
    }

    fn unary_odd_builder() -> Builder<Succ> {
        let mut builder = Builder::new();
        let states = [State::INITIAL, builder.gen_state()];

        builder
            .transition(states[0], Succ, states[1])
            .transition(states[1], Succ, states[0])
            .add_final_state(states[1]);

        builder
    }

    fn binary_odd_builder() -> Builder<bool> {
        let mut builder = Builder::new();
        let states = [State::INITIAL, builder.gen_state(), builder.gen_state()];
        builder
            .transition(states[0], false, states[1])
            .transition(states[0], true, states[2])
            .transition(states[1], false, states[1])
            .transition(states[1], true, states[2])
            .transition(states[2], false, states[1])
            .transition(states[2], true, states[2])
            .add_final_state(states[2]);
        builder
    }

    #[test]
    fn unary_odd_fast() {
        let automaton = unary_odd_builder().build_fast();
        assert!(!automaton.test(&[]));
        assert!(automaton.test(&[Succ]));
        assert!(!automaton.test(&[Succ, Succ]));
        assert!(automaton.test(&[Succ, Succ, Succ]));
    }

    #[test]
    fn unary_odd_small() {
        let automaton = dbg!(unary_odd_builder().build_small());
        assert!(!automaton.test(&[]));
        assert!(automaton.test(&[Succ]));
        assert!(!automaton.test(&[Succ, Succ]));
        assert!(automaton.test(&[Succ, Succ, Succ]));
    }

    #[test]
    fn binary_odd_fast() {
        let automaton = binary_odd_builder().build_fast();
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
    fn binary_odd_small() {
        let automaton = dbg!(binary_odd_builder().build_small());
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
}
