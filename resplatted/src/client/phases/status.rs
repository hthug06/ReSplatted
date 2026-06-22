use crate::client::core::MinecraftClient;
use base64::Engine;
use colored::{ColoredString, Colorize};
use resplatted_protocol::io::{ConnectionContext, ProtocolState, ProtocolVersion};
use resplatted_protocol::packet::{
    PacketRead,
    status::{
        c_pong_response::PongResponsePacket, c_status_response::StatusResponsePacket,
        s_ping_request::PingRequestPacket, s_status_request::StatusRequestPacket,
    },
};
use std::{
    fs::File,
    io::{Cursor, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use text_components::{TextComponent, resolving::NoResolutor};

/// For colors, legacy motd used
fn color_legacy_codes(text: &str) -> String {
    let mut result = String::new();
    let mut current_color: Option<&str> = None;
    let mut is_bold = false;

    let mut parts = text.split('§').peekable();

    if let Some(first) = parts.next() {
        result.push_str(first);
    }

    for part in parts {
        if part.is_empty() {
            continue;
        }

        let code = part.chars().next().unwrap();
        let content = &part[1..];

        match code {
            '0' => current_color = Some("black"),
            '1' => current_color = Some("blue"),
            '2' => current_color = Some("green"),
            '3' => current_color = Some("cyan"),
            '4' => current_color = Some("red"),
            '5' => current_color = Some("magenta"),
            '6' => current_color = Some("yellow"),
            '7' => current_color = Some("white"),
            '8' => current_color = Some("bright black"),
            '9' => current_color = Some("bright blue"),
            'a' => current_color = Some("bright green"),
            'b' => current_color = Some("bright cyan"),
            'c' => current_color = Some("bright red"),
            'd' => current_color = Some("bright magenta"),
            'e' => current_color = Some("bright yellow"),
            'f' => current_color = Some("bright white"),
            'l' => is_bold = true,
            'r' => {
                current_color = None;
                is_bold = false;
            }
            _ => {}
        }

        if !content.is_empty() {
            let mut colored_text: ColoredString = content.into();

            if let Some(color_name) = current_color {
                colored_text = colored_text.color(color_name);
            }
            if is_bold {
                colored_text = colored_text.bold();
            }

            result.push_str(&colored_text.to_string());
        }
    }

    result
}

/// Used to save the favicon in the `temp` directory
fn save_favicon(base64_string: &str, server_name: &str) -> std::io::Result<String> {
    let parts: Vec<&str> = base64_string.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Base64 format invalid",
        ));
    }
    let image_bytes = base64::engine::general_purpose::STANDARD
        .decode(parts[1])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    std::fs::create_dir_all(Path::new("temp"))?;

    let filepath = format!("./temp/{}_favicon.png", server_name);
    let mut file = File::create(&filepath)?;
    file.write_all(&image_bytes)?;
    Ok(filepath)
}

impl MinecraftClient {
    /// handle the status response packet and display the information in the console
    pub async fn fetch_and_display_status(
        &mut self,
        target_ip: &str,
    ) -> std::io::Result<Option<i32>> {
        // We need to define a connection context at first
        let context = ConnectionContext {
            state: ProtocolState::Handshake,
            version: ProtocolVersion::V26_1,
        };

        // Send status request
        let status_request = StatusRequestPacket;
        self.writer
            .write_and_send_packet(&status_request, &context)
            .await?;

        // Get the json
        let raw_packet = self.reader.read_packet().await?;
        let protocol_version = if raw_packet.id == StatusResponsePacket::id(&context) {
            let mut cursor = Cursor::new(raw_packet.payload.as_slice());
            let response = StatusResponsePacket::read(&mut cursor, &context)?;

            // JSON from the response of the packet
            let parsed: serde_json::Value =
                serde_json::from_str(&response.response).unwrap_or_else(|_| serde_json::json!({}));

            println!("\n==============================================");
            println!("             SERVER INFOS                     ");
            println!("==============================================");

            // Version
            let protocol_version = if let Some(version) = parsed.get("version") {
                let name = version
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let protocol = version
                    .get("protocol")
                    .and_then(|v| v.as_i64())
                    .map(|p| p.to_string())
                    .unwrap_or("Unknown".to_string());
                println!("📌 Version : {} (Protocol version : {})", name, protocol);
                protocol.parse::<i32>().unwrap_or(0)
            } else {
                println!("📌 Version : Unknown");
                ProtocolVersion::V26_1 as i32
            };

            // Player (online and max)
            if let Some(players) = parsed.get("players") {
                let online = players.get("online").and_then(|v| v.as_i64()).unwrap_or(0);
                let max = players.get("max").and_then(|v| v.as_i64()).unwrap_or(0);
                println!("👥 Players : {} / {}", online, max);
            }

            // MOTD (with snbt or legacy)
            if let Some(desc) = parsed.get("description") {
                let desc_str = desc.to_string();
                let desc_color = color_legacy_codes(&desc_str);
                if desc_color != desc_str {
                    println!("📝 MOTD    : {}", desc_color);
                } else {
                    match TextComponent::from_snbt(&desc_str) {
                        Ok(component) => {
                            println!(
                                "📝 MOTD    : {}",
                                component
                                    .to_pretty(&NoResolutor)
                                    .replace("\n", "\n              ")
                            );
                        }
                        Err(e) => {
                            log::warn!("Failed to parse SNBT for MOTD : {}", e);
                            // when an error occur, just display the normal text
                            println!("📝 MOTD    : {}", desc);
                        }
                    }
                }
            }

            // Favicon (if exist)
            if let Some(favicon) = parsed.get("favicon") {
                let base64_str = favicon.as_str().unwrap_or("");
                if let Ok(path) = save_favicon(base64_str, target_ip) {
                    println!("🖼️  Favicon : Saved in {}", path);
                } else {
                    println!(
                        "🖼️  Favicon : Server don't have favicon, or favicon is already in the folder temp/"
                    );
                }
            }
            println!("==============================================\n");
            Some(protocol_version)
        } else {
            println!("Failed to fetch status");
            None
        };

        let ping_payload = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        self.writer
            .write_and_send_packet(
                &PingRequestPacket {
                    timestamp: ping_payload,
                },
                &context,
            )
            .await?;

        let raw_pong = self.reader.read_packet().await?;
        if raw_pong.id == PongResponsePacket::id(&context) {
            let mut cursor = Cursor::new(raw_pong.payload.as_slice());
            let pong = PongResponsePacket::read(&mut cursor, &context)?;
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            println!("⚡ Latency  : {} ms\n", current_time - pong.timestamp);
        }

        Ok(protocol_version)
    }
}
