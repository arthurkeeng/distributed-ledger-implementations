use core::hash;

use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};



#[derive(Debug, Clone)]
pub struct Wallet{
    pub signing_key : SigningKey, 
    pub verifying_key : VerifyingKey, 
    pub public_key : String
}

impl Wallet{
    fn new() -> Self{
        let mut cspring = OsRng;
        let signing_key = SigningKey::generate(&mut cspring);
        let verifying_key = signing_key.verifying_key();

         Wallet {
            signing_key,
            verifying_key,
            public_key: hex::encode(verifying_key.to_bytes()),
        }
    }

    pub fn sign(&self , message : String) -> String{
        let signature = self.signing_key.sign(&message.as_bytes());
        hex::encode(signature.to_bytes())
    }

    pub fn verify (&self , message : &str , signature : &str) -> bool{
        let sig_bytes: [u8 ; 64] = hex::decode(signature).unwrap().try_into().unwrap();
        let signature = Signature::from_bytes(&sig_bytes);

        self.verifying_key.verify(message.as_bytes(), &signature).is_ok()
    }
    pub fn address (&self) -> String{
        let mut hasher = Sha256::new();

        hasher.update(self.verifying_key.to_bytes());
        format!("0x{}", hex::encode(hasher.finalize())[0..40].to_string())
    }
}

#[derive(Debug , Clone)]
pub struct SignedTransaction{
    pub from : String , 
    pub to : String , 
    pub amount : u64,
    pub signature : String , 
    pub public_key : String
}

impl SignedTransaction{
    pub fn new(from : &Wallet , to : String , amount : u64) -> Self{
        let message = format!("Send {} to {}",amount , to );
        let signature = from.sign(message);
        SignedTransaction { from: from.address(), to, amount, signature, public_key: from.public_key.clone() }
    }

        pub fn verify(&self) -> bool {
        let message = format!("Send {} to {}", self.amount, self.to);
        
        let sig_bytes: [u8 ; 64] = hex::decode(&self.signature).unwrap().try_into().unwrap();
        let pub_key_bytes = hex::decode(&self.public_key).unwrap().try_into().unwrap();
        
        let signature = Signature::from_bytes(&sig_bytes);
        let public_key = VerifyingKey::from_bytes(&pub_key_bytes).unwrap();
        
        match (signature, public_key) {
            (sig, pk) => pk.verify(message.as_bytes(), &sig).is_ok(),
            _ => false,
        }
    }
}