use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut, Range},
};

pub trait Diacritic {
    fn render(&self, position: Position) -> Option<&str>;

    fn available_pos(&self) -> OrderedPosSet;
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
            if let Some(rendered) = diacritic.render(position) {
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
pub struct OrderedPosSet {
    length: u8,
    positions: [Position; 4],
}

impl Default for OrderedPosSet {
    fn default() -> Self {
        Self::empty()
    }
}

impl OrderedPosSet {
    pub fn empty() -> Self {
        Self { length: 0, positions: [Position::Top; 4] }
    }

    pub fn new<I>(positions: I) -> Result<Self, Option<u8>>
    where
        I: IntoIterator<Item = Position>,
    {
        let mut this = Self::empty();
        for position in positions {
            this.insert(position)?;
        }
        Ok(this)
    }

    pub fn len(self) -> u8 {
        self.length
    }

    fn get_ref(&self, index: u8) -> Option<&Position> {
        if index < self.length {
            Some(&self.positions[usize::from(index)])
        } else {
            None
        }
    }

    pub fn get(self, index: u8) -> Option<Position> {
        self.get_ref(index).copied()
    }

    pub fn to_index(self, position: Position) -> Option<u8> {
        self.into_iter().position(|stored| stored == position).map(|i| i as u8)
    }

    pub fn contains(self, position: Position) -> bool {
        self.to_index(position).is_some()
    }

    pub fn insert(&mut self, position: Position) -> Result<u8, Option<u8>> {
        match self.to_index(position) {
            Some(index) => Err(Some(index)),
            None => {
                if usize::from(self.length) < self.positions.len() {
                    let index = self.length;
                    self.length += 1;
                    self.positions[usize::from(index)] = position;
                    Ok(index)
                } else {
                    Err(None)
                }
            },
        }
    }
}

impl Index<u8> for OrderedPosSet {
    type Output = Position;

    fn index(&self, index: u8) -> &Self::Output {
        #[cold]
        #[inline(never)]
        fn invalid_index(index: u8, length: u8) -> ! {
            panic!(
                "invalid ordered position set index {}, given length {}",
                index, length
            )
        }

        self.get_ref(index).unwrap_or_else(|| invalid_index(index, self.len()))
    }
}

impl IntoIterator for OrderedPosSet {
    type Item = Position;
    type IntoIter = OrderedPositions;

    fn into_iter(self) -> Self::IntoIter {
        OrderedPositions { front: 0, back: self.length, set: self }
    }
}

impl PartialEq for OrderedPosSet {
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(other.into_iter())
    }
}

impl Eq for OrderedPosSet {}

impl PartialOrd for OrderedPosSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.into_iter().partial_cmp(other.into_iter())
    }
}

impl Ord for OrderedPosSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_iter().cmp(other.into_iter())
    }
}

impl Hash for OrderedPosSet {
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
pub struct OrderedPositions {
    front: u8,
    back: u8,
    set: OrderedPosSet,
}

impl Iterator for OrderedPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let position = self.set.positions[usize::from(self.front)];
            self.front += 1;
            Some(position)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for OrderedPositions {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back > self.front {
            self.back -= 1;
            let position = self.set.positions[usize::from(self.back)];
            Some(position)
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
                    .available_pos()
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
    use crate::{solve, Diacritic, OrderedPosSet, PosMap, Position, Slot};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum PhoneticDiacritic {
        Nasalized,
        Lowered,
        Voiceless,
    }

    impl Diacritic for PhoneticDiacritic {
        fn render(&self, position: Position) -> Option<&str> {
            match (self, position) {
                (Self::Nasalized, Position::Top) => Some("\u{0303}"),
                (Self::Lowered, Position::Left) => Some("\u{02d5}"),
                (Self::Lowered, Position::Bottom) => Some("\u{031e}"),
                (Self::Voiceless, Position::Top) => Some("\u{030a}"),
                (Self::Voiceless, Position::Bottom) => Some("\u{0325}"),
                _ => None,
            }
        }

        fn available_pos(&self) -> OrderedPosSet {
            match self {
                Self::Nasalized => OrderedPosSet::new([Position::Top]).unwrap(),
                Self::Lowered => {
                    OrderedPosSet::new([Position::Bottom, Position::Left])
                        .unwrap()
                },
                Self::Voiceless => {
                    OrderedPosSet::new([Position::Bottom, Position::Top])
                        .unwrap()
                },
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
