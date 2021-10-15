use crate::transaction::Transaction;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Block{
    index: u64,
    timestamp:u64,
    payload: String,
    version: u16,
    tx: Vec<Transaction>,

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


