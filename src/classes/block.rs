use crate::utils;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::classes::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    index: u32,
    timestamp: u32,
    transactions: Vec<Transaction>,
    nonce: u32,
    hash: String,
    previous_hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: u32, transactions: Vec<Transaction>, nonce: u32, previous_hash: String) -> Self {
        let mut block = Self { 
            index,
            timestamp,
            transactions,
            nonce,
            hash: String::default(),
            previous_hash,
        };

        block.update_hash();

        block
    }

    fn to_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();

        self.index.hash(&mut hasher);
        self.timestamp.hash(&mut hasher);
        self.nonce.hash(&mut hasher);
        self.previous_hash.hash(&mut hasher);

        for transaction in &self.transactions {
            transaction.to_hash().hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    fn update_hash(&mut self) {
        self.hash = self.to_hash();
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}