use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::Hasher;
// use std::hash::Hash;

#[derive(Debug, Hash)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub prev_block_hash: String,
    pub hash: String,
    pub version: u16,
    pub nonce: u16,
    pub tx: Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: u64,
        prev_block_hash: String,
        hash: String,
        version: u16,
        tx: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        let time = chrono::offset::Utc::now();
        Self {
            index,
            timestamp: time,
            prev_block_hash,
            hash,
            version,
            nonce: 0,
            tx,
            difficulty,
        }
    }

    // adds transaction to Block.tx
    pub fn add_transaction(&mut self, new_tx: Transaction) {
        self.tx.push(new_tx);
    }

    pub fn hash_all_tx(&mut self) {
        let mut hasher = DefaultHasher::new();
        for trans in self.tx.iter() {
            trans.hash(&mut hasher)
        }
        hasher.finish();
        // self.hash = hasher;
        println!("{:?}", hasher);
        // self.hash = hasher.to_string();
    }
}

// According to https://www.gemini.com/cryptopedia/how-does-bitcoin-work-blockchain-halving#section-bitcoin-halving

// A block contains the following information:

//     A Block header:

//     Version number

//     Hash of the previous block header

//     Hash of the root of Merkle tree of all the transactions in the current block

//     Timestamp

//     Difficulty target of the current block (meaning how difficult the target hash will be to find)

//     Nonce

// The Bitcoin blockchain is maintained by a distributed network of

// anonymous peers, and in order to add a block to the blockchain,

// an individual must undergo the proof of work mining process.

// This requires that a miner (the individual participating in the mining process)

// take data from the block header as an input, and then repeatedly run it through a

//cryptographic hashing algorithm, which for Bitcoin is Secure Hash Algorithm 256 (SHA-256).

//Miners will hash slight variations of the input data, which for the mining process will be

//the nonce, until the hash of the header block results in a hash value that is less than or

//equal to the target hash value set by the network. Finding such a hash value during the

//mining process is known as a golden nonce.
