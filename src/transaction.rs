#[derive(Debug, Hash)]
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

// #[derive(Clone)]
// pub struct Output{
//     pub to_addr: Address,
//     pub value: u64,
// }

// impl Hashable for Output {
//     fn bytes (&self) -> Vec<u8> {
//         let mut bytes = vec![];

//         bytes.extend(self.to_addr.as_bytes());
//         bytes.extend(&u64_bytes(&self.value));

//         bytes
//     }
// }

// Data for each transaction:

//     Version number 

//     Flag (only for SegWit transactions)

//     Transaction inputs

//     Transaction outputs

//     Witnesses (only for SegWit transactions)

//     Lock time
