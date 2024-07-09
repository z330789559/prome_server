use std::str::FromStr;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;

pub fn validate_signature(addr: &str, signature: &str) -> bool {
   let pk=  Pubkey::try_from(addr);
    if pk.is_err(){
        return false
    }
    let pak = pk.unwrap();
    return match Signature::from_str(signature) {
        Ok(sign) => {
            if sign.verify(&pak.to_bytes()[..], b"hello world") {
                return true
            }
            false
        },
        Err(_) => false
    }
}