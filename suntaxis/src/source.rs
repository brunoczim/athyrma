#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub line: u128,
    pub column: u128,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub location: Location,
    pub length: u128,
}

impl Default for Span {
    fn default() -> Self {
        Self::from(Location::default())
    }
}

impl From<Location> for Span {
    fn from(location: Location) -> Self {
        Self { location, length: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol<T> {
    pub data: T,
    pub span: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct Source<I>
where
    I: Iterator<Item = char>,
{
    location: Location,
    input: I,
}

impl<I> Source<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Self {
        Self { location: Location::default(), input }
    }
}

impl<I> Iterator for Source<I>
where
    I: Iterator<Item = char>,
{
    type Item = Symbol<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let character = self.input.next()?;
        let span = self.location.into();
        if character == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        } else {
            self.location.column += 1;
        }
        Some(Symbol { data: character, span: Some(span) })
    }
}
