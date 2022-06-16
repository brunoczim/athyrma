use indexmap::IndexMap;
use phonetikos::Phone;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Phoneme(Phoneme),
    Allophone(Phone),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cond {
    Always,
    Never,
    Eq(Value),
    Neq(Value),
    Not(Box<Cond>),
    Any(Box<[Cond]>),
    All(Box<[Cond]>),
    Seq(Box<[Cond]>),
    Named(Box<str>, Box<Cond>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Phoneme {
    pub broad: Phone,
}

#[derive(Debug, Clone)]
pub struct PhonemeSpec {
    pub phoneme: Phoneme,
    pub allophones: IndexMap<Phone, Cond>,
}

impl PartialEq for PhonemeSpec {
    fn eq(&self, other: &Self) -> bool {
        self.phoneme == other.phoneme
            && self.allophones.iter().eq(other.allophones.iter())
    }
}

impl Eq for PhonemeSpec {}

impl PartialOrd for PhonemeSpec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PhonemeSpec {
    fn cmp(&self, other: &Self) -> Ordering {
        self.phoneme
            .cmp(&other.phoneme)
            .then_with(|| self.allophones.iter().cmp(other.allophones.iter()))
    }
}

impl Hash for PhonemeSpec {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.phoneme.hash(state);
        let mut count = 0;
        for (phone, cond) in &self.allophones {
            phone.hash(state);
            cond.hash(state);
            count += 1;
        }
        count.hash(state);
    }
}
