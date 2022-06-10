use diakritikos::{pos, slot, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Diacritic {
    Nasalized,
    Lowered,
    Voiced,
    Voiceless,
    Centralized,
}

impl diakritikos::Diacritic for Diacritic {
    fn renderings(&self) -> pos::PartialMap<&str> {
        match self {
            Diacritic::Nasalized => {
                pos::PartialMap::from_iter([(Position::Top, "\u{0303}")])
            },
            Diacritic::Lowered => pos::PartialMap::from_iter([
                (Position::Bottom, "\u{031e}"),
                (Position::Left, "\u{02d5}"),
            ]),
            Diacritic::Voiced => {
                pos::PartialMap::from_iter([(Position::Bottom, "\u{032c}")])
            },
            Diacritic::Voiceless => pos::PartialMap::from_iter([
                (Position::Bottom, "\u{0325}"),
                (Position::Top, "\u{030a}"),
            ]),
            Diacritic::Centralized => {
                pos::PartialMap::from_iter([(Position::Top, "\u{0308}")])
            },
        }
    }
}
