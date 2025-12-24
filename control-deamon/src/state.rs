use std::time::Instant;

#[derive(Debug)]
pub struct ServerState {
    pub is_running: bool,
    pub current_map: Option<String>,
    pub last_status: Option<String>,
    pub last_updated: Option<Instant>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            is_running: false,
            current_map: None,
            last_status: None,
            last_updated: None,
        }
    }
}
