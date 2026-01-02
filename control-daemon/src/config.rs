use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub rcon_host: String,
    pub rcon_port: u16,
    pub rcon_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let rcon_host = std::env::var("RCON_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let rcon_port = std::env::var("RCON_PORT")
            .unwrap_or_else(|_| "27015".into())
            .parse::<u16>()?;
        let rcon_password = std::env::var("RCON_PASSWORD")?;

        Ok(Self {
            rcon_host,
            rcon_port,
            rcon_password,
        })
    }

    pub fn rcon_addr(&self) -> String {
        format!("{}:{}", self.rcon_host, self.rcon_port)
    }
}
