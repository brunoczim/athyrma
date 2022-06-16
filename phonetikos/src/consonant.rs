use crate::{diacritic::Diacritic, Cavity, Phonation};
use diakritikos::{slot, GraphemeCluster};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Place {
    Labial,
    Alveolar,
    Velar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Manner {
    Plosive,
    Fricative,
    Approximant,
}

impl Manner {
    pub fn try_lenit(self) -> Option<Self> {
        match self {
            Self::Plosive => Some(Self::Fricative),
            Self::Fricative => Some(Self::Approximant),
            Self::Approximant => None,
        }
    }

    pub fn try_fortify(self) -> Option<Self> {
        match self {
            Self::Plosive => None,
            Self::Fricative => Some(Self::Plosive),
            Self::Approximant => Some(Self::Fricative),
        }
    }

    pub fn lenit(&mut self) {
        if let Some(manner) = self.try_lenit() {
            *self = manner;
        }
    }

    pub fn fortify(&mut self) {
        if let Some(manner) = self.try_fortify() {
            *self = manner;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Consonant {
    pub place: Place,
    pub manner: Manner,
    pub phonation: Phonation,
    pub cavity: Cavity,
    pub syllabic: bool,
}

impl Consonant {
    pub fn grapheme_cluster(self) -> GraphemeCluster<Diacritic> {
        let mut diacritics = Vec::new();
        let character = match self.manner {
            Manner::Plosive => match self.cavity {
                Cavity::Oral => match (self.place, self.phonation) {
                    (Place::Labial, Phonation::Voiceless) => 'p',
                    (Place::Labial, Phonation::Voiced) => 'b',
                    (Place::Alveolar, Phonation::Voiceless) => 't',
                    (Place::Alveolar, Phonation::Voiced) => 'd',
                    (Place::Velar, Phonation::Voiceless) => 'k',
                    (Place::Velar, Phonation::Voiced) => 'g',
                },
                Cavity::Nasal => {
                    match self.phonation {
                        Phonation::Voiceless => {
                            diacritics.push(Diacritic::Voiceless)
                        },
                        Phonation::Voiced => (),
                    }
                    match self.place {
                        Place::Labial => 'm',
                        Place::Alveolar => 'n',
                        Place::Velar => 'ŋ',
                    }
                },
            },
            Manner::Fricative => {
                match self.cavity {
                    Cavity::Nasal => diacritics.push(Diacritic::Nasalized),
                    Cavity::Oral => (),
                }
                match (self.place, self.phonation) {
                    (Place::Labial, Phonation::Voiceless) => 'ɸ',
                    (Place::Labial, Phonation::Voiced) => 'β',
                    (Place::Alveolar, Phonation::Voiceless) => 's',
                    (Place::Alveolar, Phonation::Voiced) => 'z',
                    (Place::Velar, Phonation::Voiceless) => 'x',
                    (Place::Velar, Phonation::Voiced) => 'ɣ',
                }
            },
            Manner::Approximant => {
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
                    Place::Labial => 'ʋ',
                    Place::Alveolar => 'ɹ',
                    Place::Velar => 'ɰ',
                }
            },
        };

        if self.syllabic {
            diacritics.push(Diacritic::Syllabic);
        }

        let hints = slot::hints(character).unwrap();
        GraphemeCluster::solve(character, hints, diacritics).unwrap()
    }
}

impl fmt::Display for Consonant {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.grapheme_cluster(), fmtr)
    }
}

#[cfg(test)]
mod test {
    use super::{Cavity, Consonant, Manner, Phonation, Place};

    #[test]
    fn syllabic_voiceless_nasal_labial_approximant() {
        let consonant = Consonant {
            place: Place::Labial,
            manner: Manner::Approximant,
            cavity: Cavity::Nasal,
            phonation: Phonation::Voiceless,
            syllabic: true,
        };
        assert_eq!(consonant.to_string(), "ʋ̥̩̃");
    }
}
