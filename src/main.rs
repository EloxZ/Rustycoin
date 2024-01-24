mod classes {
    pub mod transaction;
    pub mod block;
}

mod utils;

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::Rng;
use classes::transaction::Transaction;
use classes::block::Block;

fn main() {
    println!("\nRustycoin");
    let secp = Secp256k1::new();
    let mut rng = OsRng::default();
    let mut secret_key_bytes = [0u8; 32];

    println!("\nCreating keys...\n");

    rng.fill(&mut secret_key_bytes);
    let bank_sk = SecretKey::from_slice(&secret_key_bytes).expect("Invalid secret key");
    let bank_pk = PublicKey::from_secret_key(&secp, &bank_sk);
    let bank_id = 0;

    rng.fill(&mut secret_key_bytes);
    let alice_sk = SecretKey::from_slice(&secret_key_bytes).expect("Invalid secret key");
    let alice_pk = PublicKey::from_secret_key(&secp, &alice_sk);
    let alice_id = 1;

    rng.fill(&mut secret_key_bytes);
    let bob_sk = SecretKey::from_slice(&secret_key_bytes).expect("Invalid secret key");
    let bob_pk = PublicKey::from_secret_key(&secp, &bob_sk);
    let bob_id = 2;

    rng.fill(&mut secret_key_bytes);
    let joe_sk = SecretKey::from_slice(&secret_key_bytes).expect("Invalid secret key");
    let joe_pk = PublicKey::from_secret_key(&secp, &joe_sk);
    let joe_id = 3;

    println!("Bank ({}) Secret Key: {}", bank_id, bank_sk);
    println!("Bank ({}) Public Key: {}", bank_id, bank_pk);
    println!("Alice ({}) Secret Key: {}", alice_id, alice_sk);
    println!("Alice ({}) Public Key: {}", alice_id, alice_pk);
    println!("Bob ({}) Secret Key: {}", bob_id, bob_sk);
    println!("Bob ({}) Public Key: {}", bob_id, bob_pk);
    println!("Joe ({}) Secret Key: {}", joe_id, joe_sk);
    println!("Joe ({}) Public Key: {}", joe_id, joe_pk);

    println!("\nBank gives money...");

    let mut t1 = Transaction::new(bank_id, alice_id, 100.0);
    let mut t2 = Transaction::new(bank_id, bob_id, 100.0);
    let mut t3 = Transaction::new(bank_id, joe_id, 100.0);
    t1.sign_transaction(&bank_sk, &secp);
    t2.sign_transaction(&bank_sk, &secp);
    t3.sign_transaction(&bank_sk, &secp);

    println!("\n{}", t1.to_string());
    println!("Hash: {}", t1.to_hash());
    println!("Is Public Key Valid? {}", t1.is_public_key_valid(&bank_pk, &secp));
    println!("\n{}", t2.to_string());
    println!("Hash: {}", t2.to_hash());
    println!("Is Public Key Valid? {}", t2.is_public_key_valid(&bank_pk, &secp));
    println!("\n{}", t3.to_string());
    println!("Hash: {}", t3.to_hash());
    println!("Is Public Key Valid? (Must output false) {}", t3.is_public_key_valid(&alice_pk, &secp));

    println!("\nCreating block with previous transactions...\n");

    let mut transactions1: Vec<Transaction> = Vec::new();
    transactions1.push(t1);
    transactions1.push(t2);
    transactions1.push(t3);

    let mut block1 = Block::new(0, 0, transactions1, 0, String::default());
    println!("Hash: {}", block1.get_hash());


    println!("");
}
