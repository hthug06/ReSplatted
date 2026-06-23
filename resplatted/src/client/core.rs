use crate::client::network::{packet_reader::PacketReader, packet_writer::PacketWriter};
use resplatted_protocol::io::{ConnectionContext, ProtocolState, ProtocolVersion};
use resplatted_protocol::packet::handshake::{HandshakeNextState, HandshakePacket};
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct MinecraftClient {
    pub reader: PacketReader,
    pub writer: PacketWriter,
    pub context: ConnectionContext,
}

impl MinecraftClient {
    /// Create the TCP Connexion
    pub async fn connect(
        target: &str,
        port: u16,
        protocol_version: ProtocolVersion,
    ) -> std::io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", target, port)).await?;
        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: PacketReader {
                stream: BufReader::new(read_half),
                compression_threshold: None,
                raw_buffer: Vec::new(),
                decompress_buffer: Vec::new(),
            },
            writer: PacketWriter {
                stream: write_half,
                compression_threshold: None,
                raw_payload_buffer: Vec::with_capacity(32), // 32 because all the data in the packet should
                compress_buffer: Vec::with_capacity(32), // Not be more than 32 bytes (ex: connecting to
                final_buffer: Vec::with_capacity(32),    // hypixel is 22 bytes)
            },
            context: ConnectionContext {
                state: ProtocolState::Handshake,
                version: protocol_version,
            },
        })
    }

    /// Gracefully close the TCP connection (send FIN) instead of an abrupt drop.
    pub async fn disconnect(&mut self) -> std::io::Result<()> {
        self.writer.stream.shutdown().await
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
            protocol_version: self.context.version, // 26.1.2
            server_address: target_ip.to_string(),
            server_port: port,
            next_state: state_int,
        };

        self.writer
            .write_and_send_packet(&handshake, &self.context)
            .await?;

        // Update the state
        self.context.state = next_state;
        Ok(())
    }
}
