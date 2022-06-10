use crate::{Cavity, Phonation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Height {
    Open,
    Mid,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Frontness {
    Front,
    Central,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vowel {
    pub height: Height,
    pub frontness: Frontness,
    pub phonation: Phonation,
    pub cavity: Cavity,
}
