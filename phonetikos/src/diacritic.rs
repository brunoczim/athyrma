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

pub fn slot_hint(character: char) -> Option<pos::TotalMap<slot::Hint>> {
    let top_and_bottom = pos::TotalMap::default();
    let top = pos::TotalMap {
        top: slot::Hint::Obstructed,
        ..pos::TotalMap::default()
    };
    let bottom = pos::TotalMap {
        bottom: slot::Hint::Obstructed,
        ..pos::TotalMap::default()
    };
    match character {
        'a' | 'ɑ' | 'b' | 'c' | 'd' | 'e' | 'ɛ' | 'h' | 'i' | 'k' | 'm'
        | 'n' | 'o' | 'ɔ' | 'p' | 'q' | 'r' | 's' | 'u' | 'v' | 'w' | 'x'
        | 'z' => Some(top_and_bottom),
        'g' | 'j' | 'ŋ' | 'y' => Some(top),
        'f' | 'l' | 't' => Some(bottom),
        _ => None,
    }
}
