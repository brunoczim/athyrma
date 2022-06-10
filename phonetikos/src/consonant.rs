use crate::{Cavity, Phonation};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlaceOfArticulation {
    Labial,
    Alveolar,
    Velar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MannerOfArticulation {
    Plosive,
    Fricative,
    Approximant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Consonant {
    pub place: PlaceOfArticulation,
    pub manner: MannerOfArticulation,
    pub phonation: Phonation,
    pub cavity: Cavity,
}
