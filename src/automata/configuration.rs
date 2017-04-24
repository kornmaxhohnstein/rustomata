use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Configuration of an automaton containing sequence of symbols `word` to be read, a storage value `storage`, and a `weight`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Configuration<S, T, W> {
    pub word: Vec<T>,
    pub storage: S,
    pub weight: W,
}


impl<S: Hash, T: Hash, W> Hash for Configuration<S, T, W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.word.hash(state);
        self.storage.hash(state);
    }
}

impl<S: Eq, T: Eq, W: PartialOrd + Eq> PartialOrd for Configuration<S, T, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<S: Eq, T: Eq, W: Ord + Eq> Ord for Configuration<S, T, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

