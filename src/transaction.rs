#[derive(Debug)]
pub struct Transaction{
    version: u16,
    source: String,
    destination: String,
    amount: u32,
}

impl Transaction{
    pub fn new(
        version: u16,
        source: String,
        destination: String,
        amount: u32,
    ) -> Self{
        Self{
            version,
            source,
            destination,
            amount,
        }
    }
}


// Data for each transaction:

//     Version number 

//     Flag (only for SegWit transactions)

//     Transaction inputs

//     Transaction outputs

//     Witnesses (only for SegWit transactions)

//     Lock time
