use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::net::UdpSocket;

use crate::{rcon::rcon::Rcon, service::commands::router::command_router};

const LOG_PORT: u16 = 27500;

#[derive(Debug)]
pub struct ChatMessage {
    pub player: String,
    pub message: String,
    pub team_only: bool,
}

pub async fn chat_listener(rcon: Arc<dyn Rcon>) -> Result<()> {
    // Bind locally on a fixed UDP port
    let socket = UdpSocket::bind(("127.0.0.1", LOG_PORT))
        .await
        .context("failed to bind UDP socket for chat listener")?;

    let log_addr = format!("127.0.0.1:{}", LOG_PORT);

    println!("💬 Chat listener bound, advertising {}", log_addr);

    // Enable server log streaming
    let _ = rcon.exec("log on").await;
    let _ = rcon.exec("sv_logecho 1").await;
    let _ = rcon.exec("sv_logfile 1").await;
    let _ = rcon.exec("logaddress_delall").await;
    let _ = rcon.exec(&format!("logaddress_add {}", log_addr)).await;
    let mut buf = [0u8; 4096];

    loop {
        let (len, _src) = socket.recv_from(&mut buf).await?;
        let packet = String::from_utf8_lossy(&buf[..len]);

        for line in packet.lines() {
            if let Some(chat) = parse_chat_line(line) {
                println!(
                    "💬 {}{}: {}",
                    chat.player,
                    if chat.team_only { " (team)" } else { "" },
                    chat.message
                );

                handle_chat_command(&chat, Arc::clone(&rcon)).await;
            }
        }
    }
}

async fn handle_chat_command(chat: &ChatMessage, rcon: Arc<dyn Rcon>) {
    let msg = chat.message.trim();
    if !msg.starts_with('!') {
        return;
    }

    let mut parts = msg.split_whitespace();
    let command = parts.next().unwrap_or("");
    let args = parts.map(|s| s.to_string()).collect::<Vec<String>>();
    let router = command_router();

    if let Some(handler) = router.get(command) {
        handler(chat, args, rcon).await;
    }
}

fn parse_chat_line(line: &str) -> Option<ChatMessage> {
    let team_only = line.contains("\" say_team \"");
    let needle = if team_only {
        "\" say_team \""
    } else {
        "\" say \""
    };

    let colon = line.find(": \"")?;
    let after = &line[(colon + 3)..];

    let lt = after.find('<')?;
    let player = after[..lt].to_string();

    let say_pos = line.find(needle)?;
    let msg_start = say_pos + needle.len();

    let after_say = &line[msg_start..];
    let first_quote = after_say.find('"')?;
    let rest = &after_say[(first_quote + 1)..];
    let end_quote = rest.find('"')?;

    let message = rest[..end_quote].to_string();

    Some(ChatMessage {
        player,
        message,
        team_only,
    })
}
