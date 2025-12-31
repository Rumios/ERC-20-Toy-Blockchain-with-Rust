use rand::rngs::StdRng;
use rand::SeedableRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use tiny_keccak::{Keccak, Hasher};
use hex;

// Keccak-256 해시
fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

#[derive(Debug, PartialEq, Clone)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

impl Wallet {
    pub fn new_random() -> Self {
        let (address, private_key) = Self::generate_address();
        Wallet {address, private_key}
    }

    pub fn generate_address() -> (String, String) {  
        let secp = Secp256k1::new(); 
        let mut rng = StdRng::from_entropy();
        let secret_key = SecretKey::new(&mut rng as &mut dyn rand::RngCore);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let public_key_bytes = public_key.serialize_uncompressed();
        let hash = keccak256(&public_key_bytes[1..]);
        let address_bytes = &hash[12..];
        let address = format!("0x{}", hex::encode(address_bytes));  
        let private_key_hex = hex::encode(secret_key.secret_bytes());

        (address, private_key_hex)
    }
}

