use crate::utils;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, Signature, All};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    sender_id: u32,
    receiver_id: u32,
    amount: f32,
    signature: String,
}

impl Transaction {
    pub fn new(sender_id: u32, receiver_id: u32, amount: f32) -> Self {
        Self { 
            sender_id,
            receiver_id,
            amount,
            signature: String::new(),
        }
    }

    pub fn sign_transaction(&mut self, secret_key: &SecretKey, secp: &Secp256k1<All>) {
        let message = self.to_message();
        let signature = secp.sign(&message, secret_key);

        self.signature = utils::signature_to_string(&signature);
    }

    pub fn to_message(&self) -> Message {
        let transaction_data = format!("{}{}{}", self.sender_id, self.receiver_id, self.amount);
        let hash_result = Sha256::digest(transaction_data.as_bytes());

        Message::from_slice(&hash_result).expect("Error signing transaction")
    }

    pub fn to_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();

        self.sender_id.hash(&mut hasher);
        self.receiver_id.hash(&mut hasher);
        self.amount.to_bits().hash(&mut hasher);
        self.signature.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    pub fn to_string(&self) -> String {
        format!("Sender ID: {}\nReceiver ID: {}\nAmount: {} RC\nSignature: {}\n", self.sender_id, self.receiver_id, self.amount, self.signature)
    }

    pub fn is_public_key_valid(&self, public_key: &PublicKey, secp: &Secp256k1<All>) -> bool {
        let message = self.to_message();
        let validation = secp.verify(&message, &utils::string_to_signature(&self.signature), &public_key);

        validation.is_ok()
    }
}