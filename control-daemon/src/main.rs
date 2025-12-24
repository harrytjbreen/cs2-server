mod config;
mod global_state;
mod rcon_client;
mod state;
mod update_state;

use anyhow::Result;
use tokio::runtime::Builder;
use tokio::time::{Duration, sleep};

fn main() -> Result<()> {
    let rt = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let cfg = config::Config::from_env()?;
    let rcon = rcon_client::RconClient::new(cfg.rcon_addr(), cfg.rcon_password);
    let _ = global_state::init_state();

    loop {
        let _ = update_state::update_server_state(&rcon).await;

        sleep(Duration::from_secs(10)).await;
    }
}
