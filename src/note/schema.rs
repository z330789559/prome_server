use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}
