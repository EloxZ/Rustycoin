use secp256k1::{Secp256k1, Message, SecretKey, Signature, All};
use hex::FromHex;

pub fn signature_to_string(signature: &Signature) -> String {
    signature.serialize_compact()
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn string_to_signature(signature_hex_string: &String) -> Signature {
    let signature_bytes = Vec::from_hex(signature_hex_string)
        .map_err(|e| panic!("Error decoding hex string: {}", e))
        .expect("Invalid hex string");

    let signature = Signature::from_compact(&signature_bytes)
        .expect("Error converting to Signature");

    signature
}

