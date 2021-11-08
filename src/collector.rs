use tokio::net::UdpSocket;
use crate::config::Config;
use crate::{decode_message, State};
use anyhow::Result;
use anyhow::anyhow;

pub struct Collector {
    cfg: Config,
    state: State,
}

impl Collector {
    pub fn new(cfg: Config, state: State) -> Self {
        Self { cfg, state }
    }

    pub async fn run(self) -> Result<()> {
        let sock = UdpSocket::bind(self.cfg.collector_listen_address).await?;
        if self.cfg.join_mcast_group {
            if let std::net::SocketAddr::V6(a) = sock.local_addr()? {
                sock.join_multicast_v6(a.ip(), a.scope_id())?;
            } else {
                return Err(anyhow!("Joining IPv4 mcast group is not supported"));
            }
        }

        let mut buf = [0u8; 65536];

        loop {
            let (len, from) = sock.recv_from(&mut buf).await?;
            let v: serde_json::Value = decode_message(&buf[..len], self.cfg.max_msg_size)?;
            let mut state = self.state.0.write().await;
            drop(state.insert(from, v));
        }
    }
}
