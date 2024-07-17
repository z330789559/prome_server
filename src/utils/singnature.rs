use std::ffi::CString;
use std::str::FromStr;
use log::info;
use serde_derive::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::FromRow;

pub fn validate_signature(addr: &str, signature: &str,ts: TransactionSignature) -> bool {
   let pk=  Pubkey::try_from(addr);
    if pk.is_err(){
        return false
    }
    info!("origin data {:?}",ts);

    let pak = pk.unwrap();
    info!("pk {:?}",pak);
    return match Signature::from_str(signature) {
        Ok(sign) => {
            let ser=serde_json::to_string(&ts);
            if ser.is_err(){
                return false
            }
            let sign_bytes = ser.unwrap();
            info!("ser {:?}", sign_bytes.as_bytes());
            info!("16 进{:?}", hex::encode(sign_bytes.as_bytes()));
            if sign.verify(&pak.to_bytes()[..], sign_bytes.as_bytes()) {
                return true
            }
            false
        },
        Err(_) => false
    }
}


/**
{"address":"5nvCpC9TVhd8AFJ2v7kVxNxM9rWHkyukRDir1yJjZYvH",
"startTs":"1552443713",
"endTs":"1552465313",
"epvToday":"1212",
"eChgToday":"1215",
"egridinToday":"1212",
"egridoutToday":"1212",
"eDischgToday":"1212",
"cobat":"1212",
"socHvs":"1212",
"sohHvs":"1212",
"mpptPower":"1212",
"latitude":"12032.3490E",
"longitude":"3120.2075N",
}
*/
#[repr(C)]
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct  TransactionSignature{
    pub address: String,
    pub startTs: String,
    pub endTs: String,
    pub epvToday: String,
    pub eChgToday: String,
    pub egridinToday: String,
    pub egridoutToday: String,
    pub eDischgToday: String,
    pub cobat: String,
    pub socHvs: String,
    pub sohHvs: String,
    pub mpptPower: String,
    pub latitude: String,
    pub longitude: String,
}

