use crate::{global_state::get_state, rcon_client::RconClient};
use anyhow::Result;
use std::time::Instant;

pub async fn update_server_state(rcon: &RconClient) -> Result<()> {
    let result = rcon.cmd_with_retry("status", 5).await;

    let state = get_state();
    let mut state = state.write().expect("state lock poisoned");

    match result {
        Ok(output) => {
            state.last_status = Some(output);
            state.last_updated = Some(Instant::now());
        }
        Err(err) => {
            state.last_updated = Some(Instant::now());
            eprintln!("Failed to update state {err}")
        }
    }

    Ok(())
}
