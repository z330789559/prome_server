use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountSchema{
    #[serde(rename = "userName")]
    pub user_name: String,
    pub address: String,
}


#[derive(Serialize, Deserialize,Debug)]
struct LoginResponse {
    message: String,
    token: Option<String>,
}