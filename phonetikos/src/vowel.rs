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
pub enum Roundedness {
    Unrounded,
    Rounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vowel {
    pub height: Height,
    pub frontness: Frontness,
    pub roundedness: Roundedness,
    pub phonation: Phonation,
    pub cavity: Cavity,
    pub syllabic: bool,
}

impl Vowel {
    pub fn grapheme_cluster(self) -> GraphemeCluster<Diacritic> {
        let mut diacritics = Vec::new();
        let character = match (self.height, self.frontness, self.roundedness) {
            (Height::Open, Frontness::Front, Roundedness::Unrounded) => 'a',
            (Height::Open, Frontness::Front, Roundedness::Rounded) => 'ɶ',
            (Height::Open, Frontness::Central, Roundedness::Unrounded) => {
                diacritics.push(Diacritic::Centralized);
                'a'
            },
            (Height::Open, Frontness::Central, Roundedness::Rounded) => {
                diacritics.push(Diacritic::Centralized);
                'ɶ'
            },
            (Height::Open, Frontness::Back, Roundedness::Unrounded) => 'ɑ',
            (Height::Open, Frontness::Back, Roundedness::Rounded) => 'ɒ',
            (Height::Mid, Frontness::Front, Roundedness::Unrounded) => {
                diacritics.push(Diacritic::Lowered);
                'e'
            },
            (Height::Mid, Frontness::Front, Roundedness::Rounded) => {
                diacritics.push(Diacritic::Lowered);
                'ø'
            },
            (Height::Mid, Frontness::Central, Roundedness::Unrounded) => 'ə',
            (Height::Mid, Frontness::Central, Roundedness::Rounded) => {
                diacritics.push(Diacritic::Labialized);
                'ə'
            },
            (Height::Mid, Frontness::Back, Roundedness::Unrounded) => {
                diacritics.push(Diacritic::Lowered);
                'ɤ'
            },
            (Height::Mid, Frontness::Back, Roundedness::Rounded) => {
                diacritics.push(Diacritic::Lowered);
                'o'
            },
            (Height::Close, Frontness::Front, Roundedness::Unrounded) => 'i',
            (Height::Close, Frontness::Front, Roundedness::Rounded) => 'y',
            (Height::Close, Frontness::Central, Roundedness::Unrounded) => 'ɨ',
            (Height::Close, Frontness::Central, Roundedness::Rounded) => 'ʉ',
            (Height::Close, Frontness::Back, Roundedness::Unrounded) => 'ɯ',
            (Height::Close, Frontness::Back, Roundedness::Rounded) => 'u',
        };
        match self.cavity {
            Cavity::Oral => (),
            Cavity::Nasal => diacritics.push(Diacritic::Nasalized),
        }
        match self.phonation {
            Phonation::Voiced => (),
            Phonation::Voiceless => diacritics.push(Diacritic::Voiceless),
        }
        if !self.syllabic {
            diacritics.push(Diacritic::NonSyllabic);
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

#[cfg(test)]
mod test {
    use super::{Cavity, Frontness, Height, Phonation, Roundedness, Vowel};

    #[test]
    fn mid_e_nasal_voiceless_non_syllabic() {
        let vowel = Vowel {
            roundedness: Roundedness::Unrounded,
            height: Height::Mid,
            frontness: Frontness::Front,
            cavity: Cavity::Nasal,
            phonation: Phonation::Voiceless,
            syllabic: false,
        };
        assert_eq!(vowel.to_string(), "ẽ̥̯˕");
    }
}
