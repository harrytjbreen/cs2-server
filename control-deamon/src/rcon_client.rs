use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};

pub struct RconClient {
    addr: String,
    password: String,
}

impl RconClient {
    pub fn new(addr: String, password: String) -> Self {
        Self { addr, password }
    }

    pub async fn cmd(&self, command: &str) -> Result<String> {
        let mut conn = rcon::Connection::connect(&self.addr, &self.password)
            .await
            .with_context(|| format!("failed to connect to RCON at {}", self.addr))?;

        let out = conn.cmd(command).await?;
        Ok(out)
    }

    pub async fn cmd_with_retry(&self, command: &str, attempts: usize) -> Result<String> {
        let mut delay = Duration::from_millis(200);
        for i in 1..=attempts {
            match self.cmd(command).await {
                Ok(v) => return Ok(v),
                Err(e) if i < attempts => {
                    eprintln!("RCON attempt {i}/{attempts} failed: {e:#}");
                    sleep(delay).await;
                    delay = (delay * 2).min(Duration::from_secs(5));
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }
}
