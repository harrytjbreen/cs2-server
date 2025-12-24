mod config;
mod global_state;
mod rcon_client;
mod state;

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

    loop {
        match rcon.cmd_with_retry("status", 5).await {
            Ok(out) => println!("status:\n{out}"),
            Err(e) => eprintln!("status failed: {e:#}"),
        }

        sleep(Duration::from_secs(10)).await;
    }
}
