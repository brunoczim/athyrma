pub mod pos;
pub mod slot;

pub use pos::Position;
pub use slot::Slot;
use std::{collections::HashSet, fmt, ops::Range};

pub trait Diacritic {
    fn renderings(&self) -> pos::PartialMap<&str>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphemeCluster<D>
where
    D: Diacritic,
{
    pub character: char,
    pub slots: pos::TotalMap<Slot<D>>,
}

impl<D> GraphemeCluster<D>
where
    D: Diacritic,
{
    pub fn solve<I>(
        character: char,
        hints: pos::TotalMap<slot::Hint>,
        diacritics: I,
    ) -> Option<Self>
    where
        I: IntoIterator<Item = D>,
    {
        solve(hints, diacritics)
            .map(|slots| GraphemeCluster { character, slots })
    }
}

impl<D> fmt::Display for GraphemeCluster<D>
where
    D: Diacritic,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmtr, "{}", self.character)?;
        for (position, slot) in &self.slots {
            slot.fmt(position, fmtr)?;
        }
        Ok(())
    }
}

pub fn solve<D, I>(
    hints: pos::TotalMap<slot::Hint>,
    diacritics: I,
) -> Option<pos::TotalMap<Slot<D>>>
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
    hints: pos::TotalMap<slot::Hint>,
}

impl<D> Solver<D>
where
    D: Diacritic,
{
    fn new<I>(hints: pos::TotalMap<slot::Hint>, diacritics: I) -> Self
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

    fn finish(self) -> Option<pos::TotalMap<Slot<D>>> {
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
) -> HashSet<pos::TotalMap<Vec<usize>>> {
    let mut solutions = HashSet::new();
    match indices.next() {
        Some(head) => {
            let tail_solutions = build_solutions_indices(indices);
            for position in Position::ALL {
                for tail_solution in &tail_solutions {
                    let mut solution: pos::TotalMap<Vec<_>> =
                        pos::TotalMap::default();
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
            solutions.insert(pos::TotalMap::default());
        },
    }
    solutions
}

#[derive(Debug, Clone)]
struct Solution {
    points: u128,
    slot_indices: pos::TotalMap<Vec<usize>>,
}

impl Solution {
    fn new<D>(
        slot_indices: pos::TotalMap<Vec<usize>>,
        diacritics: &[D],
        slot_hints: pos::TotalMap<slot::Hint>,
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
                slot::Hint::Regular => 0,
                slot::Hint::Obstructed => {
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
    pub slots: pos::TotalMap<Slot<D>>,
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
    use crate::{pos, solve, Diacritic, Position, Slot};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum PhoneticDiacritic {
        Nasalized,
        Lowered,
        Voiceless,
    }

    impl Diacritic for PhoneticDiacritic {
        fn renderings(&self) -> pos::PartialMap<&str> {
            match self {
                PhoneticDiacritic::Nasalized => {
                    pos::PartialMap::from_iter([(Position::Top, "\u{0303}")])
                },
                PhoneticDiacritic::Lowered => pos::PartialMap::from_iter([
                    (Position::Bottom, "\u{031e}"),
                    (Position::Left, "\u{02d5}"),
                ]),
                PhoneticDiacritic::Voiceless => pos::PartialMap::from_iter([
                    (Position::Bottom, "\u{0325}"),
                    (Position::Top, "\u{030a}"),
                ]),
            }
        }
    }

    #[test]
    fn single_diacritic() {
        let solution =
            solve(pos::TotalMap::default(), [PhoneticDiacritic::Nasalized])
                .unwrap();
        assert_eq!(solution, pos::TotalMap {
            top: Slot { diacritics: vec![PhoneticDiacritic::Nasalized] },
            left: Slot { diacritics: Vec::new() },
            bottom: Slot { diacritics: Vec::new() },
            right: Slot { diacritics: Vec::new() },
        });
    }

    #[test]
    fn triple_diacritic() {
        let solution = solve(pos::TotalMap::default(), [
            PhoneticDiacritic::Nasalized,
            PhoneticDiacritic::Lowered,
            PhoneticDiacritic::Voiceless,
        ])
        .unwrap();

        assert_eq!(solution, pos::TotalMap {
            top: Slot { diacritics: vec![PhoneticDiacritic::Nasalized] },
            left: Slot { diacritics: vec![PhoneticDiacritic::Lowered] },
            bottom: Slot { diacritics: vec![PhoneticDiacritic::Voiceless] },
            right: Slot { diacritics: Vec::new() },
        });
    }
}
