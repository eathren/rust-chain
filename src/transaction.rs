#[derive(Debug)]
pub struct Transaction{
    version: u16,
    source: String,
    destination: String,
    amount: u32,
}



// Data for each transaction:

//     Version number 

//     Flag (only for SegWit transactions)

//     Transaction inputs

//     Transaction outputs

//     Witnesses (only for SegWit transactions)

//     Lock time
