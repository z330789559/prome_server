mod singnature;
pub use singnature::*;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

pub fn serialize_exact<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    value.to_string().serialize(serializer)
}
pub fn deserialize_from_string<'de,S>(deserializer: S)->Result<BigDecimal,S::Error>
    where
        S:serde::de::Deserializer<'de>{

    let s = String::deserialize(deserializer)?;
    Ok(BigDecimal::from_str(&s).unwrap())
}

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}