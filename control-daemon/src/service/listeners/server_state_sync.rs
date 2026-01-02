use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::interval;

use crate::rcon::rcon::Rcon;
use crate::state::parse_server_status::parse_server_status;
use crate::state::state::ServerState;

pub async fn sync_state(rcon: Arc<dyn Rcon>, status_tx: watch::Sender<Option<ServerState>>) {
    let mut tick = interval(Duration::from_secs(1));

    loop {
        tick.tick().await;

        let server_status = match rcon.exec("status").await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to fetch server status: {e:#}");
                continue;
            }
        };

        let new_state = parse_server_status(&server_status);

        let _ = status_tx.send(Some(new_state));
    }
}
