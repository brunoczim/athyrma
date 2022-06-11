use crate::{diacritic::Diacritic, Cavity, Phonation};
use diakritikos::{slot, GraphemeCluster};
use std::fmt::{self, Debug};

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

impl Vowel {
    pub fn grapheme_cluster(self) -> GraphemeCluster<Diacritic> {
        let mut diacritics = Vec::new();
        let character = match (self.height, self.frontness) {
            (Height::Open, Frontness::Front) => 'a',
            (Height::Open, Frontness::Central) => {
                diacritics.push(Diacritic::Centralized);
                'a'
            },
            (Height::Open, Frontness::Back) => 'ɑ',
            (Height::Mid, Frontness::Front) => {
                diacritics.push(Diacritic::Lowered);
                'e'
            },
            (Height::Mid, Frontness::Central) => 'ə',
            (Height::Mid, Frontness::Back) => {
                diacritics.push(Diacritic::Lowered);
                'o'
            },
            (Height::Close, Frontness::Front) => 'i',
            (Height::Close, Frontness::Central) => 'ɨ',
            (Height::Close, Frontness::Back) => 'u',
        };
        match self.cavity {
            Cavity::Oral => (),
            Cavity::Nasal => diacritics.push(Diacritic::Nasalized),
        }
        match self.phonation {
            Phonation::Voiced => (),
            Phonation::Voiceless => diacritics.push(Diacritic::Voiceless),
        }
        let hints = slot::hints(character).unwrap();
        GraphemeCluster::solve(character, hints, diacritics).unwrap()
    }
}

impl fmt::Display for Vowel {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.grapheme_cluster(), fmtr)
    }
}
