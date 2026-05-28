use crate::client::{
    network::{packet_reader::PacketReader, packet_writer::PacketWriter},
    state::ProtocolState,
};
use resplatted_protocol::packet::handshake::{HandshakeNextState, HandshakePacket};
use tokio::io::BufReader;
use tokio::net::TcpStream;

pub struct MinecraftClient {
    pub reader: PacketReader,
    pub writer: PacketWriter,
    pub state: ProtocolState,
}

impl MinecraftClient {
    /// Create the TCP Connexion
    pub async fn connect(target: &str, port: u16) -> std::io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", target, port)).await?;
        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: PacketReader {
                stream: BufReader::new(read_half),
                compression_threshold: None,
            },
            writer: PacketWriter {
                stream: write_half,
                compression_threshold: None,
            },
            state: ProtocolState::Handshake,
        })
    }

    /// Handshake function for the client
    /// Used both for the status and the login
    pub async fn handshake(
        &mut self,
        target_ip: &str,
        port: u16,
        next_state: ProtocolState,
    ) -> std::io::Result<()> {
        let state_int = match next_state {
            ProtocolState::Status => HandshakeNextState::Status,
            ProtocolState::Login => HandshakeNextState::Login,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid Next State",
                ));
            }
        };

        let handshake = HandshakePacket {
            protocol_version: 775, // 26.1.2
            server_address: target_ip.to_string(),
            server_port: port,
            next_state: state_int,
        };

        self.writer.write_and_send_packet(&handshake).await?;

        // Update the state
        self.state = next_state;
        Ok(())
    }
}
