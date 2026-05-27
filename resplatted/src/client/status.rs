use super::core::MinecraftClient;
use base64::Engine;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use resplatted_protocol::packet::PacketRead;
use resplatted_protocol::packet::status::{
    c_pong_response::PongResponsePacket, c_status_response::StatusResponsePacket,
    s_ping_request::PingRequestPacket, s_status_request::StatusRequestPacket,
};
/// Because favicon use § with a char for color, we need to skip them
fn clean_motd(motd: &str) -> String {
    let mut cleaned = String::new();
    let mut skip_next = false;
    for c in motd.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if c == '§' {
            skip_next = true;
        } else {
            cleaned.push(c);
        }
    }
    cleaned
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

    match std::fs::create_dir(Path::new("temp")) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    let filepath = format!("./temp/{}_favicon.png", server_name);
    let mut file = File::create(&filepath)?;
    file.write_all(&image_bytes)?;
    Ok(filepath)
}

impl MinecraftClient {
    /// handle the status response packet and display the information in the console
    pub async fn fetch_and_display_status(&mut self, target_ip: &str) -> std::io::Result<()> {
        // Send status request
        let status_request = StatusRequestPacket;
        self.writer.write_packet(&status_request).await?;

        // Get the json
        let raw_packet = self.reader.read_packet().await?;
        if raw_packet.id == 0x00 {
            let mut cursor = Cursor::new(raw_packet.payload.as_slice());
            let response = StatusResponsePacket::read(&mut cursor)?;

            let parsed: serde_json::Value =
                serde_json::from_str(&response.response).unwrap_or_else(|_| serde_json::json!({}));

            println!("\n==============================================");
            println!("             SERVER INFOS                     ");
            println!("==============================================");

            if let Some(version) = parsed.get("version") {
                let name = version
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                println!("📌 Version : {}", name);
            }

            if let Some(players) = parsed.get("players") {
                let online = players.get("online").and_then(|v| v.as_i64()).unwrap_or(0);
                let max = players.get("max").and_then(|v| v.as_i64()).unwrap_or(0);
                println!("👥 Players : {} / {}", online, max);
            }

            if let Some(desc) = parsed.get("description") {
                let raw_desc = desc.as_str().unwrap_or("");
                println!(
                    "📝 MOTD    : {}",
                    clean_motd(raw_desc).replace("\n", "\n                  ")
                );
            }

            if let Some(favicon) = parsed.get("favicon") {
                let base64_str = favicon.as_str().unwrap_or("");
                if let Ok(path) = save_favicon(base64_str, target_ip) {
                    println!("🖼️  Favicon : Save in {}", path);
                } else {
                    println!(
                        "🖼️  Favicon : Server don't have favicon, or favicon is already in the folder temp/"
                    );
                }
            }
            println!("==============================================\n");
        }

        // 3. Le Ping / Pong pour la latence
        let ping_payload = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        self.writer
            .write_packet(&PingRequestPacket {
                timestamp: ping_payload,
            })
            .await?;

        let raw_pong = self.reader.read_packet().await?;
        if raw_pong.id == 0x01 {
            let mut cursor = Cursor::new(raw_pong.payload.as_slice());
            let pong = PongResponsePacket::read(&mut cursor)?;
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            println!("⚡ Latency  : {} ms\n", current_time - pong.timestamp);
        }

        Ok(())
    }
}
