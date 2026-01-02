mod config;
mod rcon;
pub mod service;
mod state;

use std::sync::Arc;

use anyhow::Result;
use tokio::runtime::Builder;
use tokio::sync::watch;

use crate::rcon::client::RconClient;
use crate::rcon::rcon::Rcon;
use crate::service::listeners::chat_listener::chat_listener;
use crate::service::listeners::server_state_sync::sync_state;
use crate::state::state::ServerState;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let rt = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let cfg = config::Config::from_env()?;

    let rcon: Arc<dyn Rcon> = Arc::new(RconClient::new(cfg.rcon_addr(), cfg.rcon_password));

    let (status_tx, _) = watch::channel::<Option<ServerState>>(None);

    tokio::spawn(sync_state(Arc::clone(&rcon), status_tx));
    tokio::spawn(chat_listener(Arc::clone(&rcon)));

    tokio::signal::ctrl_c().await?;
    Ok(())
}
