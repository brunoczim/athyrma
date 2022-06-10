pub mod diacritic;
pub mod vowel;
pub mod consonant;

use consonant::Consonant;
use std::fmt;
use vowel::Vowel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phonation {
    Voiceless,
    Voiced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cavity {
    Nasal,
    Oral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phone {
    Consonant(Consonant),
    Vowel(Vowel),
}

impl fmt::Display for Phone {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Phone::Vowel(vowel) => fmt::Display::fmt(vowel, fmtr),
            Phone::Consonant(consonant) => fmt::Display::fmt(consonant, fmtr),
        }
    }
}
