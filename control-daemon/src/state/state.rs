use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    pub last_status: Option<String>,
    pub last_updated: Option<Instant>,
    pub map: Option<String>,
    pub num_of_players: Option<u32>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            last_status: None,
            last_updated: None,
            map: None,
            num_of_players: None,
        }
    }
}
