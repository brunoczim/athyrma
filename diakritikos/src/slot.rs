use crate::{
    pos::{self, Position},
    Diacritic,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hint {
    Regular,
    Obstructed,
}

impl Default for Hint {
    fn default() -> Self {
        Hint::Regular
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot<D>
where
    D: Diacritic,
{
    pub diacritics: Vec<D>,
}

impl<D> Default for Slot<D>
where
    D: Diacritic,
{
    fn default() -> Self {
        Self { diacritics: Vec::new() }
    }
}

impl<D> Slot<D>
where
    D: Diacritic,
{
    pub(crate) fn fmt(
        &self,
        position: Position,
        fmtr: &mut fmt::Formatter,
    ) -> fmt::Result {
        for diacritic in &self.diacritics {
            if let Some(rendered) = diacritic.renderings().data(position) {
                write!(fmtr, "{}", rendered)?;
            }
        }
        Ok(())
    }
}

pub fn hints(character: char) -> Option<pos::TotalMap<Hint>> {
    let top_and_bottom = pos::TotalMap::default();
    let top =
        pos::TotalMap { top: Hint::Obstructed, ..pos::TotalMap::default() };
    let bottom =
        pos::TotalMap { bottom: Hint::Obstructed, ..pos::TotalMap::default() };
    match character {
        'a' | 'ɑ' | 'b' | 'c' | 'd' | 'e' | 'ɛ' | 'h' | 'i' | 'k' | 'm'
        | 'n' | 'o' | 'ɔ' | 'p' | 'q' | 'r' | 'ɹ' | 's' | 'u' | 'v' | 'ʋ'
        | 'w' | 'ɰ' | 'x' | 'z' => Some(top_and_bottom),
        'g' | 'j' | 'ŋ' | 'y' => Some(top),
        'f' | 'l' | 't' => Some(bottom),
        _ => None,
    }
}
