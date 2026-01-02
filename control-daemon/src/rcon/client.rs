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
        let mut attempts = 0;

        loop {
            attempts += 1;

            let connect = timeout(
                self.timeout,
                rcon::Connection::connect(&self.addr, &self.password),
            )
            .await;

            let mut conn = match connect {
                Ok(Ok(conn)) => conn,

                Ok(Err(_e)) => {
                    if attempts >= 10 {
                        return Err(anyhow::anyhow!(
                            "failed to connect to RCON at {} after {} attempts",
                            self.addr,
                            attempts
                        ));
                    }

                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }

                Err(_elapsed) => {
                    if attempts >= 10 {
                        return Err(anyhow::anyhow!(
                            "timeout connecting to RCON at {} after {} attempts",
                            self.addr,
                            attempts
                        ));
                    }

                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            };

            // Command execution
            let out = timeout(self.timeout, conn.cmd(command))
                .await
                .context("timeout executing RCON command")??;

            return Ok(out);
        }
    }
}
