use std::ffi::CString;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

use crate::utils::{serialize_exact, deserialize_from_string, deserialize_i64_from_string, TransactionSignature};

#[derive( Deserialize, Debug,Clone)]
pub struct  CreateTransactionSchema {
    pub address: String,
    #[serde(rename = "startTs",deserialize_with = "deserialize_i64_from_string")]
    pub start_ts: i64,
    #[serde(rename = "endTs",deserialize_with = "deserialize_i64_from_string")]
    pub end_ts: i64,
    #[serde(rename = "epvToday",deserialize_with = "deserialize_from_string")]
    pub epv_today: BigDecimal,
    #[serde(rename = "eChgToday",deserialize_with = "deserialize_from_string")]
    pub e_chg_today: BigDecimal,
    #[serde(rename = "egridinToday",deserialize_with = "deserialize_from_string")]
    pub egridin_today: BigDecimal,
    #[serde(rename = "egridoutToday",deserialize_with = "deserialize_from_string")]
    pub egridout_today: BigDecimal,
    #[serde(rename = "eDischgToday",deserialize_with = "deserialize_from_string")]
    pub e_dischg_today: BigDecimal,
    #[serde(rename = "cobat",deserialize_with = "deserialize_from_string")]
    pub cobat: BigDecimal,
    #[serde(rename = "socHvs",deserialize_with = "deserialize_from_string")]
    pub soc_hvs: BigDecimal,
    #[serde(rename = "sohHvs",deserialize_with = "deserialize_from_string")]
    pub soh_hvs: BigDecimal,
    #[serde(rename = "mpptPower",deserialize_with = "deserialize_from_string")]
    pub mppt_power: BigDecimal,
    pub latitude: String,
    pub longitude: String,
    pub signature: String,
}

impl From<CreateTransactionSchema> for TransactionSignature{
    fn from(value: CreateTransactionSchema) -> Self {
        unsafe {
            return TransactionSignature {
                address: value.address,
                startTs: value.start_ts.to_string(),
                endTs: value.end_ts.to_string(),
                epvToday: value.epv_today.to_string(),
                eChgToday: value.e_chg_today.to_string(),
                egridinToday: value.egridin_today.to_string(),
                egridoutToday: value.egridout_today.to_string(),
                eDischgToday: value.e_dischg_today.to_string(),
                cobat: value.cobat.to_string(),
                socHvs: value.soc_hvs.to_string(),
                sohHvs: value.soh_hvs.to_string(),
                mpptPower: value.mppt_power.to_string(),
                latitude: value.latitude,
                longitude: value.longitude,
            }
        }
    }
}