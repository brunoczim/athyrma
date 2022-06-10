use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut, Range},
};

pub trait Diacritic {
    fn renderings(&self) -> OrderedPosMap<&str>;
}

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
        [Self::Top, Self::Left, Self::Bottom, Self::Right];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SlotHint {
    Regular,
    Obstructed,
}

impl Default for SlotHint {
    fn default() -> Self {
        SlotHint::Regular
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
    fn fmt(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PosMap<T> {
    pub top: T,
    pub left: T,
    pub bottom: T,
    pub right: T,
}

impl<T> PosMap<T> {
    pub fn from_fn<F>(mut function: F) -> Self
    where
        F: FnMut(Position) -> T,
    {
        PosMap {
            top: function(Position::Top),
            left: function(Position::Left),
            bottom: function(Position::Bottom),
            right: function(Position::Right),
        }
    }

    pub fn as_ref(&self) -> PosMap<&T> {
        PosMap {
            top: &self.top,
            left: &self.left,
            bottom: &self.bottom,
            right: &self.right,
        }
    }

    pub fn as_mut(&mut self) -> PosMap<&mut T> {
        PosMap {
            top: &mut self.top,
            left: &mut self.left,
            bottom: &mut self.bottom,
            right: &mut self.right,
        }
    }

    pub fn map<F, U>(self, mut mapper: F) -> PosMap<U>
    where
        F: FnMut(T) -> U,
    {
        self.map_with_pos(|_, elem| mapper(elem))
    }

    pub fn map_with_pos<F, U>(self, mut mapper: F) -> PosMap<U>
    where
        F: FnMut(Position, T) -> U,
    {
        PosMap {
            top: mapper(Position::Top, self.top),
            left: mapper(Position::Left, self.left),
            bottom: mapper(Position::Bottom, self.bottom),
            right: mapper(Position::Right, self.right),
        }
    }
}

impl<T> PosMap<Option<T>> {
    pub fn transpose(self) -> Option<PosMap<T>> {
        Some(PosMap {
            top: self.top?,
            left: self.left?,
            bottom: self.bottom?,
            right: self.right?,
        })
    }
}

impl<T> Index<Position> for PosMap<T> {
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

impl<T> IndexMut<Position> for PosMap<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        match index {
            Position::Top => &mut self.top,
            Position::Bottom => &mut self.bottom,
            Position::Left => &mut self.left,
            Position::Right => &mut self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrderedPosMap<T> {
    length: u8,
    positions: [Option<(Position, T)>; 4],
}

impl<T> Default for OrderedPosMap<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> OrderedPosMap<T> {
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

    pub fn iter(&self) -> OrderedPosMapIter<T> {
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

impl<T> FromIterator<(Position, T)> for OrderedPosMap<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (Position, T)>,
    {
        Self::new(iterable).expect("duplicated positions")
    }
}

impl<'map, T> IntoIterator for &'map OrderedPosMap<T> {
    type Item = (Position, &'map T);
    type IntoIter = OrderedPosMapIter<'map, T>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedPosMapIter { front: 0, back: self.length, map: self }
    }
}

impl<T> IntoIterator for OrderedPosMap<T> {
    type Item = (Position, T);
    type IntoIter = OrderedPosMapIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedPosMapIntoIter { front: 0, back: self.length, map: self }
    }
}

impl<T> PartialEq for OrderedPosMap<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(other.into_iter())
    }
}

impl<T> Eq for OrderedPosMap<T> where T: Eq {}

impl<T> PartialOrd for OrderedPosMap<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.into_iter().partial_cmp(other.into_iter())
    }
}

impl<T> Ord for OrderedPosMap<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_iter().cmp(other.into_iter())
    }
}

impl<T> Hash for OrderedPosMap<T>
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
pub struct OrderedPosMapIntoIter<T> {
    front: u8,
    back: u8,
    map: OrderedPosMap<T>,
}

impl<T> Iterator for OrderedPosMapIntoIter<T> {
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

impl<T> DoubleEndedIterator for OrderedPosMapIntoIter<T> {
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
pub struct OrderedPosMapIter<'map, T> {
    front: u8,
    back: u8,
    map: &'map OrderedPosMap<T>,
}

impl<'map, T> Iterator for OrderedPosMapIter<'map, T> {
    type Item = (Position, &'map T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let (position, data) =
                self.map.positions[usize::from(self.back)].as_ref().unwrap();
            self.front += 1;
            Some((*position, data))
        } else {
            None
        }
    }
}

impl<'map, T> DoubleEndedIterator for OrderedPosMapIter<'map, T> {
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

pub fn solve<D, I>(
    hints: PosMap<SlotHint>,
    diacritics: I,
) -> Option<PosMap<Slot<D>>>
where
    I: IntoIterator<Item = D>,
    D: Diacritic,
{
    Solver::new(hints, diacritics).finish()
}

#[derive(Debug, Clone)]
struct Solver<D>
where
    D: Diacritic,
{
    diacritics: Vec<D>,
    hints: PosMap<SlotHint>,
}

impl<D> Solver<D>
where
    D: Diacritic,
{
    fn new<I>(hints: PosMap<SlotHint>, diacritics: I) -> Self
    where
        I: IntoIterator<Item = D>,
    {
        let diacritics = diacritics.into_iter().collect();
        Self { diacritics, hints }
    }

    fn find_solution(&self) -> Option<Solution> {
        let mut best_solution: Option<Solution> = None;
        let raw_solutions = build_solutions_indices(0 .. self.diacritics.len());
        for raw_solution in raw_solutions {
            if let Some(solution) =
                Solution::new(raw_solution, &self.diacritics, self.hints)
            {
                match &best_solution {
                    Some(found_solution) => {
                        if solution.points < found_solution.points {
                            best_solution = Some(solution);
                        }
                    },
                    None => best_solution = Some(solution),
                }
            }
        }
        best_solution
    }

    fn finish(self) -> Option<PosMap<Slot<D>>> {
        let best_solution = self.find_solution()?;
        let mut diacritics: Vec<_> =
            self.diacritics.into_iter().map(Some).collect();
        Some(best_solution.slot_indices.map(|indices| {
            Slot {
                diacritics: indices
                    .into_iter()
                    .map(|index| diacritics[index].take().unwrap())
                    .collect(),
            }
        }))
    }
}

fn build_solutions_indices(
    mut indices: Range<usize>,
) -> HashSet<PosMap<Vec<usize>>> {
    let mut solutions = HashSet::new();
    match indices.next() {
        Some(head) => {
            let tail_solutions = build_solutions_indices(indices);
            for position in Position::ALL {
                for tail_solution in &tail_solutions {
                    let mut solution: PosMap<Vec<_>> = PosMap::default();
                    solution[position] = vec![head];
                    for tail_position in Position::ALL {
                        solution[tail_position]
                            .extend_from_slice(&tail_solution[tail_position]);
                    }
                    solutions.insert(solution);
                }
            }
        },
        None => {
            solutions.insert(PosMap::default());
        },
    }
    solutions
}

#[derive(Debug, Clone)]
struct Solution {
    points: u128,
    slot_indices: PosMap<Vec<usize>>,
}

impl Solution {
    fn new<D>(
        slot_indices: PosMap<Vec<usize>>,
        diacritics: &[D],
        slot_hints: PosMap<SlotHint>,
    ) -> Option<Self>
    where
        D: Diacritic,
    {
        let mut points = 0;
        let mean = (diacritics.len() + 2) / 4;
        for position in Position::ALL {
            let mut maybe_previous_index = None;
            for index in slot_indices[position].iter() {
                points += diacritics[*index]
                    .renderings()
                    .to_index(position)
                    .map(u128::from)?
                    * 2;
                if let Some(previous_index) = maybe_previous_index {
                    if previous_index > *index {
                        points += 1;
                    }
                }
                maybe_previous_index = Some(*index);
            }
            points += match slot_hints[position] {
                SlotHint::Regular => 0,
                SlotHint::Obstructed => {
                    u128::try_from(slot_indices[position].len()).unwrap()
                },
            };
            points +=
                u128::try_from(slot_indices[position].len().abs_diff(mean))
                    .unwrap()
                    * 4;
        }

        Some(Self { points, slot_indices })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol<D>
where
    D: Diacritic,
{
    pub character: char,
    pub slots: PosMap<Slot<D>>,
}

impl<D> fmt::Display for Symbol<D>
where
    D: Diacritic,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        self.slots.left.fmt(Position::Left, fmtr)?;
        write!(fmtr, "{}", self.character)?;
        self.slots.left.fmt(Position::Top, fmtr)?;
        self.slots.left.fmt(Position::Bottom, fmtr)?;
        self.slots.left.fmt(Position::Right, fmtr)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{solve, Diacritic, OrderedPosMap, PosMap, Position, Slot};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum PhoneticDiacritic {
        Nasalized,
        Lowered,
        Voiceless,
    }

    impl Diacritic for PhoneticDiacritic {
        fn renderings(&self) -> OrderedPosMap<&str> {
            match self {
                PhoneticDiacritic::Nasalized => {
                    OrderedPosMap::from_iter([(Position::Top, "\u{0303}")])
                },
                PhoneticDiacritic::Lowered => OrderedPosMap::from_iter([
                    (Position::Bottom, "\u{031e}"),
                    (Position::Left, "\u{02d5}"),
                ]),
                PhoneticDiacritic::Voiceless => OrderedPosMap::from_iter([
                    (Position::Bottom, "\u{0325}"),
                    (Position::Top, "\u{030a}"),
                ]),
            }
        }
    }

    #[test]
    fn single_diacritic() {
        let solution =
            solve(PosMap::default(), [PhoneticDiacritic::Nasalized]).unwrap();
        assert_eq!(
            solution,
            PosMap {
                top: Slot { diacritics: vec![PhoneticDiacritic::Nasalized] },
                left: Slot { diacritics: Vec::new() },
                bottom: Slot { diacritics: Vec::new() },
                right: Slot { diacritics: Vec::new() },
            }
        );
    }

    #[test]
    fn triple_diacritic() {
        let solution = solve(
            PosMap::default(),
            [
                PhoneticDiacritic::Nasalized,
                PhoneticDiacritic::Lowered,
                PhoneticDiacritic::Voiceless,
            ],
        )
        .unwrap();

        assert_eq!(
            solution,
            PosMap {
                top: Slot { diacritics: vec![PhoneticDiacritic::Nasalized] },
                left: Slot { diacritics: vec![PhoneticDiacritic::Lowered] },
                bottom: Slot { diacritics: vec![PhoneticDiacritic::Voiceless] },
                right: Slot { diacritics: Vec::new() },
            }
        );
    }
}
