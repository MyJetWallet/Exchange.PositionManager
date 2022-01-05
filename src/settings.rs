use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "sb_host_port")]
    pub sb_host_port: String,

    #[serde(rename = "app_name")]
    pub app_name: String
}

impl Settings {
    pub async fn from_file(path: String) -> Settings{
        let content = fs::read_to_string(path).await.unwrap();
        return serde_json::from_str(&content).unwrap();
    }
}