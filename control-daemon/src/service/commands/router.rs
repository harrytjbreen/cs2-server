use std::collections::HashMap;

use crate::service::commands::CommandHandler;
use crate::service::commands::{map, ping, restart};

pub fn command_router() -> HashMap<&'static str, CommandHandler> {
    let mut map: HashMap<&'static str, CommandHandler> = HashMap::new();

    map.insert("!ping", ping::ping as CommandHandler);
    map.insert("!map", map::map as CommandHandler);
    map.insert("!restart", restart::restart as CommandHandler);

    map
}
