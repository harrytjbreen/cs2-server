use crate::rcon::rcon::Rcon;
use anyhow::{Context, Result};
use tokio::time::{Duration, timeout};

pub struct RconClient {
    addr: String,
    password: String,
    timeout: Duration,
}

impl RconClient {
    pub fn new(addr: String, password: String) -> Self {
        Self {
            addr,
            password,
            timeout: Duration::from_secs(3),
        }
    }
}

#[async_trait::async_trait]
impl Rcon for RconClient {
    async fn exec(&self, command: &str) -> Result<String> {
        let mut conn = timeout(
            self.timeout,
            rcon::Connection::connect(&self.addr, &self.password),
        )
        .await
        .context("timeout connecting to RCON")?
        .with_context(|| format!("failed to connect to RCON at {}", self.addr))?;

        let out = timeout(self.timeout, conn.cmd(command))
            .await
            .context("timeout executing RCON command")??;

        Ok(out)
    }
}
