use std::sync::{Arc, OnceLock, RwLock};

use crate::state::ServerState;

static GLOBAL_STATE: OnceLock<Arc<RwLock<ServerState>>> = OnceLock::new();

pub fn init_state() -> Arc<RwLock<ServerState>> {
    let state = Arc::new(RwLock::new(ServerState::new()));
    GLOBAL_STATE
        .set(state.clone())
        .expect("Global State already initialised");
    state
}

pub fn get_state() -> Arc<RwLock<ServerState>> {
    GLOBAL_STATE
        .get()
        .expect("Global State not initalised")
        .clone()
}
