use serde::Deserialize;
use tokio::fs;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub collector_listen_address: String,
    pub http_listen_address: String,
    pub join_mcast_group: bool,
    pub max_msg_size: usize,
}

impl Config {
    pub async fn from_path(path: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(&fs::read_to_string(path).await?)?)
    }
}
