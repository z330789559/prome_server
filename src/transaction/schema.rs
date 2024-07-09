
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

use crate::utils::{serialize_exact,deserialize_from_string};
#[derive( Deserialize, Debug)]
pub struct  CreateTransactionSchema {
    pub addr: String,
    #[serde(rename = "epvToday",deserialize_with = "deserialize_from_string")]
    pub epv_today: BigDecimal,
    #[serde(rename = "eChgToday",deserialize_with = "deserialize_from_string")]
    pub e_chg_today: BigDecimal,
    #[serde(rename = "eDischgToday",deserialize_with = "deserialize_from_string")]
    pub e_dischg_today: BigDecimal,
    pub signature: String,
}