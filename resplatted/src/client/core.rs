use crate::client::network::{PacketReader, PacketWriter};
use crate::client::state::ProtocolState;
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
    pub async fn connect(address: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address).await?;
        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: PacketReader {
                stream: BufReader::new(read_half),
            },
            writer: PacketWriter { stream: write_half },
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
            protocol_version: 775, // 1.20.1
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
