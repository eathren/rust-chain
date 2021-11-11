use crate::transaction::Transaction;
use std::fmt::{  Debug };
use sha2::{Sha256};
use chrono::{DateTime,  Utc};

#[derive(Debug)]
pub struct Block{
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub prev_block_hash: String,
    pub hash: String,
    pub version: u16,
    pub nonce: u16,
    pub tx: Vec<Transaction>,
    pub difficulty: u128,
}


impl Block{
    pub fn new(
        index:u64, 
        prev_block_hash: String,
        hash: String,
        version: u16,
        tx: Vec<Transaction>,
        difficulty: u128
    ) -> Self{
        let time = chrono::offset::Utc::now();
        Self{
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
    pub fn add_transaction(&mut self, new_tx:Transaction) {
        self.tx.push(new_tx);
    }
}

    // pub fn mine( &mut self){
    //     for nonce_attempt in 0..(u64::max_value()){
    //         self.nonce = nonce_attempt;
    //         let hash = self.hash();
    //         if check_difficulty(&hash, self.difficulty){
    //             self.hash = hash;
    //             return;
    //         }
    //     }
    // }
   
// pub fn check_difficulty (hash:  &Hash, difficulty: u128) -> bool {
//     difficulty > difficulty_bytes_as_u128(&hash)
// }

// impl Hashable for Block{
//     fn bytes (&self) -> Vec<u8> {
//         let mut bytes = vec![];

//         bytes.extend(&u32_bytes(&self.index));
//         bytes.extend(&u64_bytes(&self.timestamp));
//         bytes.extend(&self.prev_block_hash);
//         bytes.extend(&u64_bytes(&self.nonce));
//         bytes.extend(
//             self.transactions
//             .iter()
//             .flat_map(|transaction| transaction.bytes())
//             .collect::<Vec<u8>>()
//         );
//         bytes.extend(&u128_bytes(&self.difficulty));

//         bytes


//     }
// }



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