// use crate::hashable::Hashable;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, Hash)]
pub struct Transaction {
    version: u16,
    source: String,
    destination: String,
    amount: u32,
}

impl Transaction {
    pub fn new(version: u16, source: String, destination: String, amount: u32) -> Self {
        Self {
            version,
            source,
            destination,
            amount,
        }
    }

    pub fn hash_tx<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash<H: Hasher>(&self, state: &mut H) {
        self.version.hash(state);
        self.source.hash(state);
        self.destination.hash(state);
        self.amount.hash(state);
    }
}

// impl Hash for Transaction {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.version.hash(state);
//         self.source.hash(state);
//         self.destination.hash(state);
//         self.amount.hash(state);
//     }
// }

// Data for each transaction:

//     Version number

//     Flag (only for SegWit transactions)

//     Transaction inputs

//     Transaction outputs

//     Witnesses (only for SegWit transactions)

//     Lock time
