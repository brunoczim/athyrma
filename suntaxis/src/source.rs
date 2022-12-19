use core::fmt;

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

impl fmt::Display for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Default for Span {
    fn default() -> Self {
        Self::from(Location::default())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        if self.start() == self.inclusive_end() {
            write!(fmtr, "{}", self.start())
        } else {
            write!(fmtr, "{} - {}", self.start(), self.inclusive_end())
        }
    }
}

impl Span {
    pub fn new_inclusive(start: Location, end: Location) -> Self {
        Self::try_new_inclusive(start, end)
            .expect("end must not come before start")
    }

    pub fn new(start: Location, end: Location) -> Self {
        Self::try_new(start, end).expect("end must not come before start")
    }

    pub fn try_new(start: Location, end: Location) -> Result<Self, SpanError> {
        if end >= start {
            Ok(Self { start, end })
        } else {
            Err(SpanError { given_start: start, given_end: end })
        }
    }

    pub fn try_new_inclusive(
        start: Location,
        end: Location,
    ) -> Result<Self, SpanError> {
        let converted_end = Location { line: end.line, column: end.column + 1 };
        if converted_end >= start {
            Ok(Self::new(start, converted_end))
        } else {
            Err(SpanError { given_start: start, given_end: end })
        }
    }

    pub fn start(self) -> Location {
        self.start
    }

    pub fn end(self) -> Location {
        self.end
    }

    pub fn inclusive_end(self) -> Location {
        Location { line: self.end.line, column: self.end.column - 1 }
    }
}

impl From<Location> for Span {
    fn from(location: Location) -> Self {
        Self::new(location, Location {
            line: location.line,
            column: location.column + 1,
        })
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

#[derive(Debug, Clone)]
pub struct SpanError {
    pub given_start: Location,
    pub given_end: Location,
}

impl fmt::Display for SpanError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Span's end {} must not come before span's start {}",
            self.given_end, self.given_start
        )
    }
}
