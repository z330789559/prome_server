use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use sqlx::FromRow;
use sqlx::types::BigDecimal;
use uuid::Uuid;
use crate::utils::{serialize_exact,deserialize_from_string,serialize_i64_to_string,deserialize_i64_from_string};


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
"signature":"66iS3TFHzW33mH11w1nT1SGG7xZHUsvmt8sQHMvQEeGL6basgDTrWZiMjdTMkfYUzshRX6d3NfH6dAvFi2U5KMef"}
*/
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct  TransactionModel{
    pub id: i32,
    pub addr: String,
    #[serde(rename = "startTs", serialize_with = "serialize_i64_to_string",deserialize_with = "deserialize_i64_from_string")]
    pub start_ts: i64,
    #[serde(rename = "endTs", serialize_with = "serialize_i64_to_string",deserialize_with = "deserialize_i64_from_string")]
    pub end_ts: i64,
    #[serde(rename = "epvToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub epv_today: BigDecimal,
    #[serde(rename = "eChgToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub e_chg_today: BigDecimal,
    #[serde(rename = "egridinToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub egridin_today: BigDecimal,
    #[serde(rename = "egridoutToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub egridout_today: BigDecimal,
    #[serde(rename = "eDischgToday", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub e_dischg_today: BigDecimal,
    #[serde(rename = "cobat", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub cobat: BigDecimal,
    #[serde(rename = "socHvs", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub soc_hvs: BigDecimal,
    #[serde(rename = "sohHvs", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub soh_hvs: BigDecimal,
    #[serde(rename = "mpptPower", serialize_with = "serialize_exact",deserialize_with = "deserialize_from_string")]
    pub mppt_power: BigDecimal,
    pub latitude: String,
    pub longitude: String,
    pub signature: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}

