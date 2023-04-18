pub trait Countable {
    const COUNT: usize;
}

impl Countable for () {
    const COUNT: usize = 0x1;
}

impl<A, B> Countable for (A, B)
where
    A: Countable,
    B: Countable,
{
    const COUNT: usize = A::COUNT * B::COUNT;
}

impl Countable for bool {
    const COUNT: usize = 0x2;
}

impl Countable for u8 {
    const COUNT: usize = 0x100;
}

impl Countable for i8 {
    const COUNT: usize = 0x100;
}

#[cfg(not(target_pointer_width = "16"))]
impl Countable for u16 {
    const COUNT: usize = 0x100_00;
}

#[cfg(not(target_pointer_width = "16"))]
impl Countable for i16 {
    const COUNT: usize = 0x100_00;
}

pub trait ToIndex: Countable {
    fn to_index(&self) -> usize;
}

impl ToIndex for () {
    fn to_index(&self) -> usize {
        0
    }
}

impl<A, B> ToIndex for (A, B)
where
    A: ToIndex,
    B: ToIndex,
{
    fn to_index(&self) -> usize {
        self.0.to_index() * B::COUNT + self.1.to_index()
    }
}

impl ToIndex for bool {
    fn to_index(&self) -> usize {
        usize::from(*self)
    }
}

impl ToIndex for u8 {
    fn to_index(&self) -> usize {
        usize::from(*self)
    }
}

impl ToIndex for i8 {
    fn to_index(&self) -> usize {
        usize::from(*self as u8)
    }
}

#[cfg(not(target_pointer_width = "16"))]
impl ToIndex for u16 {
    fn to_index(&self) -> usize {
        usize::from(*self)
    }
}

#[cfg(not(target_pointer_width = "16"))]
impl ToIndex for i16 {
    fn to_index(&self) -> usize {
        usize::from(*self as u16)
    }
}

pub trait FromIndex: Countable {
    fn from_index(index: usize) -> Self;
}

impl FromIndex for () {
    fn from_index(_index: usize) -> Self {
        ()
    }
}

impl<A, B> FromIndex for (A, B)
where
    A: FromIndex,
    B: FromIndex,
{
    fn from_index(index: usize) -> Self {
        (A::from_index(index / B::COUNT), B::from_index(index % B::COUNT))
    }
}

impl FromIndex for bool {
    fn from_index(index: usize) -> Self {
        [false, true][index]
    }
}

impl FromIndex for u8 {
    fn from_index(index: usize) -> Self {
        index as Self
    }
}

impl FromIndex for i8 {
    fn from_index(index: usize) -> Self {
        index as Self
    }
}

#[cfg(not(target_pointer_width = "16"))]
impl FromIndex for u16 {
    fn from_index(index: usize) -> Self {
        index as Self
    }
}

#[cfg(not(target_pointer_width = "16"))]
impl FromIndex for i16 {
    fn from_index(index: usize) -> Self {
        index as Self
    }
}

pub trait Indexable: ToIndex + FromIndex {}

impl<T> Indexable for T where T: ToIndex + FromIndex + ?Sized {}
