mod command;
mod worker;

use std::collections::HashSet;
use std::hash::Hash;

pub use command::*;
pub use worker::*;

#[cfg(test)]
mod worker_mock;
#[cfg(test)]
pub use worker_mock::tests::*;

/// A representation for Linux distribution information for the system.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Distribution {
    pub id: String,
    pub release: String,
}

/// Return true if the two (potentially unordered) vecs contain identical elements.
pub fn vecs_eq<T>(v1: Vec<T>, v2: Vec<T>) -> bool
where
    T: Hash + Eq,
{
    if v1.len() != v2.len() {
        return false;
    }
    let hs: HashSet<_> = v1.iter().collect();
    v2.iter().all(|i| hs.contains(i))
}
