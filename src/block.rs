use crate::transaction::Transaction;
// use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Block{
    index: u64,
    timestamp:u64,
    payload: String,
    version: u16,
    nonce: u16,
    merkle: u32,
    pub tx: Vec<Transaction>,
}

impl Block{
    pub fn new(
        index:u64, 
        timestamp:u64,
        payload: String,
        version: u16,
        nonce: u16,
        merkle: u32,
        tx: Vec<Transaction>,
    ) -> Self{
        Self{
            index,
            timestamp,
            payload,
            version,
            nonce,
            merkle,
            tx
        }
    }

    // adds transaction to Block.tx
    pub fn add_transaction(&mut self, new_tx:Transaction) {
        self.tx.push(new_tx);
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