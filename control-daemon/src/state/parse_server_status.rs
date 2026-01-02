use crate::state::{
    state::ServerState,
    util::{extract_map_name, extract_number_before},
};

pub fn parse_server_status(input: &str) -> ServerState {
    let mut state = ServerState::new();

    let mut humans: Option<u32> = None;
    let mut bots: Option<u32> = None;

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with("hostname") {
            state.last_status = Some("Running".to_string());
        }

        if line.starts_with("players") && line.contains("humans") {
            humans = extract_number_before(line, "humans");
            bots = extract_number_before(line, "bots");
        }

        if line.contains("spawngroup") && line.contains("de_") {
            if let Some(map) = extract_map_name(line) {
                state.map = Some(map);
            }
        }
    }

    if let (Some(h), Some(b)) = (humans, bots) {
        state.num_of_players = Some(h + b);
    }

    state
}
