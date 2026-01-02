mod map;
mod ping;
mod restart;
pub mod router;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::rcon::rcon::Rcon;
use crate::service::listeners::chat_listener::ChatMessage;

pub type CommandFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub type CommandHandler = fn(&ChatMessage, Vec<String>, Arc<dyn Rcon>) -> CommandFuture;
