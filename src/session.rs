use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Deserialize, Serialize)]
pub struct TimeFormat {
    pub format_12_hour: String,
    pub format_24_hour: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Data {
    Array(Vec<Session>),
    Object(HashMap<String, Session>),
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub data: Data,
}

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub starts_at: TimeFormat,
    pub ends_at: TimeFormat,
    pub spaces: u32,
}