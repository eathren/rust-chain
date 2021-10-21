mod block;
mod transaction;
use crate::block::Block;

fn main() {
    println!("Hello, world!");
    let block = Block::new( 10, 10, String::from("Test"),  4, 8, 7, Vec::new());
    println!("{:?}", block)
}
