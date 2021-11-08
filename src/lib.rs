mod collector;
mod server;
mod config;

pub use server::Server;
pub use config::Config;
pub use collector::Collector;

use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

use tokio::sync::RwLock;
use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use miniz_oxide::inflate::decompress_to_vec_with_limit;

#[derive(Default, Debug, Clone)]
pub struct State (Arc<RwLock<HashMap<SocketAddr, serde_json::Value>>>);

pub fn decode_message<T>(msg: &[u8], max_size: usize) -> Result<T> where T: DeserializeOwned {
    let decompressed = decompress_to_vec_with_limit(msg, max_size).map_err(|e| anyhow!("{:?}", e))?;
    Ok(serde_json::from_reader(&*decompressed)?)
}
