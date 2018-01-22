
use time;
use time::Timespec;
use time::get_time;
use sha2::{Sha256, Digest};

pub fn hello()
{
    println!("hello block!");
}

// an instance of a transaction
pub struct Transaction
{
    sender: Sha256,
    recipient: Sha256,
    amount: i32,
}

// a block in the block chain
pub struct BlockElem
{
    pub index: u32,
    timestamp: Timespec, // timestamp is number of seconds since 1970
    transactions: Vec<Transaction>,
    proof: u64,
    last_hash: Sha256,
}

impl BlockElem
{
    // constructor method
    pub fn new(nextIndex:u32,last_block_hash:&Sha256) -> BlockElem
    {
        BlockElem {index: nextIndex,timestamp:get_time(),proof: 1,last_hash:last_block_hash.clone(),transactions:Vec::new()}
    }

    pub fn add_transaction(&mut self,a_sender:&Sha256,a_recipient:&Sha256,a_amount:i32)
    {
        self.transactions.push(Transaction {sender:a_sender.clone() , recipient:a_recipient.clone(),amount:a_amount});
    }

    pub fn list_block(&self)
    {
        for i in &(self.transactions)
        {
            
        }
    }
}
