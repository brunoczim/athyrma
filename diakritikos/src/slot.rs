use crate::{pos::Position, Diacritic};
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
