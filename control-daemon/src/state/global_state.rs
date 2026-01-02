use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

use crate::state::state::ServerState;
#[allow(dead_code)]
static GLOBAL_STATE: OnceLock<Arc<RwLock<ServerState>>> = OnceLock::new();

#[allow(dead_code)]
fn state() -> &'static Arc<RwLock<ServerState>> {
    GLOBAL_STATE.get_or_init(|| Arc::new(RwLock::new(ServerState::new())))
}

#[allow(dead_code)]
pub async fn read_state<R>(f: impl FnOnce(&ServerState) -> R) -> R {
    let guard = state().read().await;
    f(&guard)
}

#[allow(dead_code)]
pub async fn write_state<R>(f: impl FnOnce(&mut ServerState) -> R) -> R {
    let mut guard = state().write().await;
    f(&mut guard)
}
