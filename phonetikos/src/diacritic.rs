use diakritikos::{OrderedPosMap, PosMap, Position, SlotHint};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Diacritic {
    Nasalized,
    Lowered,
    Voiced,
    Voiceless,
    Centralized,
}

impl diakritikos::Diacritic for Diacritic {
    fn renderings(&self) -> OrderedPosMap<&str> {
        match self {
            Diacritic::Nasalized => {
                OrderedPosMap::from_iter([(Position::Top, "\u{0303}")])
            },
            Diacritic::Lowered => OrderedPosMap::from_iter([
                (Position::Bottom, "\u{031e}"),
                (Position::Left, "\u{02d5}"),
            ]),
            Diacritic::Voiced => {
                OrderedPosMap::from_iter([(Position::Bottom, "\u{032c}")])
            },
            Diacritic::Voiceless => OrderedPosMap::from_iter([
                (Position::Bottom, "\u{0325}"),
                (Position::Top, "\u{030a}"),
            ]),
            Diacritic::Centralized => {
                OrderedPosMap::from_iter([(Position::Top, "\u{0308}")])
            },
        }
    }
}

pub fn slot_hint(character: char) -> Option<PosMap<SlotHint>> {
    let top_and_bottom = PosMap::default();
    let top = PosMap { top: SlotHint::Obstructed, ..PosMap::default() };
    let bottom = PosMap { bottom: SlotHint::Obstructed, ..PosMap::default() };
    match character {
        'a' | 'ɑ' | 'b' | 'c' | 'd' | 'e' | 'ɛ' | 'h' | 'i' | 'k' | 'm'
        | 'n' | 'o' | 'ɔ' | 'p' | 'q' | 'r' | 's' | 'u' | 'v' | 'w' | 'x'
        | 'z' => Some(top_and_bottom),
        'g' | 'j' | 'ŋ' | 'y' => Some(top),
        'f' | 'l' | 't' => Some(bottom),
        _ => None,
    }
}
