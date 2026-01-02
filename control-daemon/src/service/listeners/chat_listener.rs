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
    // 🔧 Bind wide to ensure CS2 can reach us
    let socket = UdpSocket::bind(("0.0.0.0", LOG_PORT))
        .await
        .context("failed to bind UDP socket for chat listener")?;

    let log_addr = format!("127.0.0.1:{}", LOG_PORT);

    println!(
        "💬 Chat listener bound on 0.0.0.0:{}, advertising {}",
        LOG_PORT, log_addr
    );

    // ---- RCON setup with logging ----
    exec_logged(&rcon, "log on").await;
    exec_logged(&rcon, "sv_logecho 1").await;
    exec_logged(&rcon, "sv_logfile 1").await;
    exec_logged(&rcon, "logaddress_delall").await;
    exec_logged(&rcon, &format!("logaddress_add {}", log_addr)).await;

    println!("📡 Waiting for UDP log packets…");

    let mut buf = [0u8; 4096];

    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        println!("📥 UDP packet received: {} bytes from {}", len, src);

        let packet = String::from_utf8_lossy(&buf[..len]);

        for line in packet.lines() {
            println!("🧾 RAW LOG: {}", line);

            match parse_chat_line(line) {
                Some(chat) => {
                    println!(
                        "💬 PARSED CHAT {}{}: {}",
                        chat.player,
                        if chat.team_only { " (team)" } else { "" },
                        chat.message
                    );

                    handle_chat_command(&chat, Arc::clone(&rcon)).await;
                }
                None => {
                    println!("🔍 Ignored non-chat line");
                }
            }
        }
    }
}

async fn exec_logged(rcon: &Arc<dyn Rcon>, cmd: &str) {
    match rcon.exec(cmd).await {
        Ok(_) => println!("✅ RCON OK: {}", cmd),
        Err(err) => println!("❌ RCON FAIL: {} → {:?}", cmd, err),
    }
}

async fn handle_chat_command(chat: &ChatMessage, rcon: Arc<dyn Rcon>) {
    let msg = chat.message.trim();

    if !msg.starts_with('!') {
        println!("↪️  Not a command, ignoring");
        return;
    }

    let mut parts = msg.split_whitespace();
    let command = parts.next().unwrap_or("");
    let args = parts.map(|s| s.to_string()).collect::<Vec<String>>();

    println!("⚙️  Command detected: {} {:?}", command, args);

    let router = command_router();

    if let Some(handler) = router.get(command) {
        handler(chat, args, rcon).await;
    } else {
        println!("❓ Unknown command: {}", command);
    }
}

fn parse_chat_line(line: &str) -> Option<ChatMessage> {
    // CS:GO-style detection (may not match CS2 — logging will confirm)
    let team_only = line.contains("\" say_team \"");
    let needle = if team_only {
        "\" say_team \""
    } else {
        "\" say \""
    };

    let colon = match line.find(": \"") {
        Some(v) => v,
        None => return None,
    };

    let after = &line[(colon + 3)..];

    let lt = match after.find('<') {
        Some(v) => v,
        None => return None,
    };

    let player = after[..lt].to_string();

    let say_pos = match line.find(needle) {
        Some(v) => v,
        None => return None,
    };

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
