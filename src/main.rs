mod block;
mod transaction;
use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transaction = Transaction::new(10, String::from("me"), String::from("you"), 10000);
    // Should blocks be mutable? Usually Crypto is not. 
    let mut block = Block::new( 
        10, 
        String::from("de95704ab706215d4450a055284c82bedcd52353dcd5239280d6adbb21752d23"), 
        String::from("Test"),  
        1, 
        Vec::with_capacity(1000),
        100
    );
    block.add_transaction(transaction);
    println!("{:?}", block);
}
