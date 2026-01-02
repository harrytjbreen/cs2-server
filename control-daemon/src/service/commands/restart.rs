use std::sync::Arc;

use crate::rcon::rcon::Rcon;
use crate::service::commands::CommandFuture;
use crate::service::listeners::chat_listener::ChatMessage;

pub fn restart(_chat: &ChatMessage, _args: Vec<String>, rcon: Arc<dyn Rcon>) -> CommandFuture {
    Box::pin(async move {
        let _ = rcon.exec("mp_restartgame 1").await;
    })
}
