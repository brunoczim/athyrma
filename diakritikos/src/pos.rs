use std::{
    array,
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Position {
    Top,
    Left,
    Bottom,
    Right,
}

impl Position {
    pub const ALL: [Self; 4] =
        [Self::Left, Self::Top, Self::Bottom, Self::Right];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TotalMap<T> {
    pub top: T,
    pub left: T,
    pub bottom: T,
    pub right: T,
}

impl<T> TotalMap<T> {
    pub fn from_fn<F>(mut function: F) -> Self
    where
        F: FnMut(Position) -> T,
    {
        TotalMap {
            top: function(Position::Top),
            left: function(Position::Left),
            bottom: function(Position::Bottom),
            right: function(Position::Right),
        }
    }

    pub fn as_ref(&self) -> TotalMap<&T> {
        TotalMap {
            top: &self.top,
            left: &self.left,
            bottom: &self.bottom,
            right: &self.right,
        }
    }

    pub fn as_mut(&mut self) -> TotalMap<&mut T> {
        TotalMap {
            top: &mut self.top,
            left: &mut self.left,
            bottom: &mut self.bottom,
            right: &mut self.right,
        }
    }

    pub fn map<F, U>(self, mut mapper: F) -> TotalMap<U>
    where
        F: FnMut(T) -> U,
    {
        self.map_with_pos(|_, elem| mapper(elem))
    }

    pub fn map_with_pos<F, U>(self, mut mapper: F) -> TotalMap<U>
    where
        F: FnMut(Position, T) -> U,
    {
        TotalMap {
            top: mapper(Position::Top, self.top),
            left: mapper(Position::Left, self.left),
            bottom: mapper(Position::Bottom, self.bottom),
            right: mapper(Position::Right, self.right),
        }
    }

    pub fn iter(&self) -> TotalMapIntoIter<&T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> TotalMapIntoIter<&mut T> {
        self.into_iter()
    }
}

impl<T> TotalMap<Option<T>> {
    pub fn transpose(self) -> Option<TotalMap<T>> {
        Some(TotalMap {
            top: self.top?,
            left: self.left?,
            bottom: self.bottom?,
            right: self.right?,
        })
    }
}

impl<T> Index<Position> for TotalMap<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        match index {
            Position::Top => &self.top,
            Position::Bottom => &self.bottom,
            Position::Left => &self.left,
            Position::Right => &self.right,
        }
    }
}

impl<T> IndexMut<Position> for TotalMap<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        match index {
            Position::Top => &mut self.top,
            Position::Bottom => &mut self.bottom,
            Position::Left => &mut self.left,
            Position::Right => &mut self.right,
        }
    }
}

impl<T> IntoIterator for TotalMap<T> {
    type Item = (Position, T);
    type IntoIter = TotalMapIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        TotalMapIntoIter {
            positions: Position::ALL.into_iter(),
            map: self.map(Some),
        }
    }
}

impl<'data, T> IntoIterator for &'data TotalMap<T> {
    type Item = (Position, &'data T);
    type IntoIter = TotalMapIntoIter<&'data T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'data, T> IntoIterator for &'data mut TotalMap<T> {
    type Item = (Position, &'data mut T);
    type IntoIter = TotalMapIntoIter<&'data mut T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_iter()
    }
}

pub struct TotalMapIntoIter<T> {
    positions: array::IntoIter<Position, 4>,
    map: TotalMap<Option<T>>,
}

impl<T> Iterator for TotalMapIntoIter<T> {
    type Item = (Position, T);

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.positions.next()?;
        Some((position, self.map[position].take().unwrap()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PartialMap<T> {
    length: u8,
    positions: [Option<(Position, T)>; 4],
}

impl<T> Default for PartialMap<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> PartialMap<T> {
    pub fn empty() -> Self {
        Self { length: 0, positions: [None, None, None, None] }
    }

    pub fn new<I>(positions: I) -> Result<Self, Option<u8>>
    where
        I: IntoIterator<Item = (Position, T)>,
    {
        let mut this = Self::empty();
        for (position, data) in positions {
            this.insert(position, data)?;
        }
        Ok(this)
    }

    pub fn len(&self) -> u8 {
        self.length
    }

    pub fn iter(&self) -> PartialMapIter<T> {
        self.into_iter()
    }

    pub fn data(&self, position: Position) -> Option<&T> {
        self.index_data(self.to_index(position)?)
    }

    pub fn index_position(&self, index: u8) -> Option<Position> {
        self.index_entry(index).map(|(position, _)| position)
    }

    pub fn index_data(&self, index: u8) -> Option<&T> {
        self.index_entry(index).map(|(_, data)| data)
    }

    pub fn index_entry(&self, index: u8) -> Option<(Position, &T)> {
        self.positions.get(usize::from(index)).and_then(|option_entry| {
            let (position, value) = option_entry.as_ref()?;
            Some((*position, value))
        })
    }

    pub fn to_index(&self, position: Position) -> Option<u8> {
        self.iter().position(|(stored, _)| stored == position).map(|i| i as u8)
    }

    pub fn contains(&self, position: Position) -> bool {
        self.to_index(position).is_some()
    }

    pub fn insert(
        &mut self,
        position: Position,
        data: T,
    ) -> Result<u8, Option<u8>> {
        match self.to_index(position) {
            Some(index) => Err(Some(index)),
            None => {
                if usize::from(self.length) < self.positions.len() {
                    let index = self.length;
                    self.length += 1;
                    self.positions[usize::from(index)] = Some((position, data));
                    Ok(index)
                } else {
                    Err(None)
                }
            },
        }
    }
}

impl<T> FromIterator<(Position, T)> for PartialMap<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (Position, T)>,
    {
        Self::new(iterable).expect("duplicated positions")
    }
}

impl<'map, T> IntoIterator for &'map PartialMap<T> {
    type Item = (Position, &'map T);
    type IntoIter = PartialMapIter<'map, T>;

    fn into_iter(self) -> Self::IntoIter {
        PartialMapIter { front: 0, back: self.length, map: self }
    }
}

impl<T> IntoIterator for PartialMap<T> {
    type Item = (Position, T);
    type IntoIter = PartialMapIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        PartialMapIntoIter { front: 0, back: self.length, map: self }
    }
}

impl<T> PartialEq for PartialMap<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(other.into_iter())
    }
}

impl<T> Eq for PartialMap<T> where T: Eq {}

impl<T> PartialOrd for PartialMap<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.into_iter().partial_cmp(other.into_iter())
    }
}

impl<T> Ord for PartialMap<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_iter().cmp(other.into_iter())
    }
}

impl<T> Hash for PartialMap<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut length = 0u8;
        for position in self.into_iter() {
            length += 1;
            position.hash(state);
        }
        length.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct PartialMapIntoIter<T> {
    front: u8,
    back: u8,
    map: PartialMap<T>,
}

impl<T> Iterator for PartialMapIntoIter<T> {
    type Item = (Position, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let entry =
                self.map.positions[usize::from(self.front)].take().unwrap();
            self.front += 1;
            Some(entry)
        } else {
            None
        }
    }
}

impl<T> DoubleEndedIterator for PartialMapIntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back > self.front {
            self.back -= 1;
            let entry =
                self.map.positions[usize::from(self.back)].take().unwrap();
            Some(entry)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartialMapIter<'map, T> {
    front: u8,
    back: u8,
    map: &'map PartialMap<T>,
}

impl<'map, T> Iterator for PartialMapIter<'map, T> {
    type Item = (Position, &'map T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let (position, data) =
                self.map.positions[usize::from(self.front)].as_ref().unwrap();
            self.front += 1;
            Some((*position, data))
        } else {
            None
        }
    }
}

impl<'map, T> DoubleEndedIterator for PartialMapIter<'map, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back > self.front {
            self.back -= 1;
            let (position, data) =
                self.map.positions[usize::from(self.back)].as_ref().unwrap();
            Some((*position, data))
        } else {
            None
        }
    }
}
