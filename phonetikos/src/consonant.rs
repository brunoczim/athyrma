use crate::{diacritic::Diacritic, Cavity, Phonation};
use diakritikos::{slot, GraphemeCluster};
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

impl Consonant {
    pub fn grapheme_cluster(self) -> GraphemeCluster<Diacritic> {
        let mut diacritics = Vec::new();
        let character = match self.manner {
            MannerOfArticulation::Plosive => match self.cavity {
                Cavity::Oral => match (self.place, self.phonation) {
                    (PlaceOfArticulation::Labial, Phonation::Voiceless) => 'p',
                    (PlaceOfArticulation::Labial, Phonation::Voiced) => 'b',
                    (PlaceOfArticulation::Alveolar, Phonation::Voiceless) => {
                        't'
                    },
                    (PlaceOfArticulation::Alveolar, Phonation::Voiced) => 'd',
                    (PlaceOfArticulation::Velar, Phonation::Voiceless) => 'k',
                    (PlaceOfArticulation::Velar, Phonation::Voiced) => 'g',
                },
                Cavity::Nasal => {
                    match self.phonation {
                        Phonation::Voiceless => {
                            diacritics.push(Diacritic::Voiceless)
                        },
                        Phonation::Voiced => (),
                    }
                    match self.place {
                        PlaceOfArticulation::Labial => 'm',
                        PlaceOfArticulation::Alveolar => 'n',
                        PlaceOfArticulation::Velar => 'ŋ',
                    }
                },
            },
            MannerOfArticulation::Fricative => {
                match self.cavity {
                    Cavity::Nasal => diacritics.push(Diacritic::Nasalized),
                    Cavity::Oral => (),
                }
                match (self.place, self.phonation) {
                    (PlaceOfArticulation::Labial, Phonation::Voiceless) => 'ɸ',
                    (PlaceOfArticulation::Labial, Phonation::Voiced) => 'β',
                    (PlaceOfArticulation::Alveolar, Phonation::Voiceless) => {
                        's'
                    },
                    (PlaceOfArticulation::Alveolar, Phonation::Voiced) => 'z',
                    (PlaceOfArticulation::Velar, Phonation::Voiceless) => 'x',
                    (PlaceOfArticulation::Velar, Phonation::Voiced) => 'ɣ',
                }
            },
            MannerOfArticulation::Approximant => {
                match self.phonation {
                    Phonation::Voiced => (),
                    Phonation::Voiceless => {
                        diacritics.push(Diacritic::Voiceless)
                    },
                }
                match self.cavity {
                    Cavity::Nasal => diacritics.push(Diacritic::Nasalized),
                    Cavity::Oral => (),
                }
                match self.place {
                    PlaceOfArticulation::Labial => 'm',
                    PlaceOfArticulation::Alveolar => 'n',
                    PlaceOfArticulation::Velar => 'ŋ',
                }
            },
        };
        let hints = slot::hints(character).unwrap();
        GraphemeCluster::solve(character, hints, diacritics).unwrap()
    }
}

impl fmt::Display for Consonant {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.grapheme_cluster(), fmtr)
    }
}
