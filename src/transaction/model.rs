use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use sqlx::FromRow;
use sqlx::types::BigDecimal;
use uuid::Uuid;
use crate::utils::{serialize_exact,deserialize_from_string};


/**
*{
    "addr":BQpPHJHL2yg3PDdJPXu6h8RFguEqnAH31x68ptTkYmXA,
    "epvToday":1212,
    "eChgToday":1212,
    "eDischgToday":1212,
    "signature":sign(addr-eDischgToday)
}
*/
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct  TransactionModel{
    pub id: Uuid,
    pub addr: String,
    #[serde(rename = "epvToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub epv_today: BigDecimal,
    #[serde(rename = "eChgToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub e_chg_today: BigDecimal,
    #[serde(rename = "eDischgToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub e_dischg_today: BigDecimal,
    pub signature: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}

