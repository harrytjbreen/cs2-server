use std::sync::Arc;

use crate::rcon::rcon::Rcon;
use crate::service::commands::CommandFuture;
use crate::service::listeners::chat_listener::ChatMessage;

pub fn map(_chat: &ChatMessage, args: Vec<String>, rcon: Arc<dyn Rcon>) -> CommandFuture {
    Box::pin(async move {
        if let Some(map) = args.first() {
            let _ = rcon.exec(&format!("changelevel {}", map)).await;
        } else {
            let _ = rcon.exec("say Usage: !map <mapname>").await;
        }
    })
}
