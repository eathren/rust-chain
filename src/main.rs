mod block;
mod transaction;
use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transaction = Transaction::new(10, String::from("me"), String::from("you"), 10000);
    // Should blocks be mutable? Usually Crypto is not. 
    // let mut block = Block::new( 10, 10, String::from("Test"),  4, 8, 7, Vec::with_capacity(1000));
    // block.add_transaction(transaction);
    // block.hash();
    // println!("{:?}", block);
}
