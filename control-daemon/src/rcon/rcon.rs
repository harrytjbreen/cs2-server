use anyhow::Result;

#[async_trait::async_trait]
pub trait Rcon: Send + Sync {
    async fn exec(&self, command: &str) -> Result<String>;
}
