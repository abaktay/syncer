use crate::id::Id;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

const DISCOVERY_PORT: u16 = 2828;

// broadcasting packets to identify the receiver
pub async fn peer_broadcast(socket: &UdpSocket, my_id: &Id) -> std::io::Result<()> {
    let payload = my_id.to_bytes();
    // let broadcast_addr = (Ipv4Addr::LOCALHOST, DISCOVERY_PORT); // localhost for debugging
    let broadcast_addr = (Ipv4Addr::BROADCAST, DISCOVERY_PORT);
    socket.send_to(&payload, broadcast_addr).await?;
    Ok(())
}

pub async fn listen_for_peers(socket: &UdpSocket) -> std::io::Result<Id> {
    let mut buf = [0u8; 1024];
    let (len, _addr) = socket.recv_from(&mut buf).await?;
    Id::from_bytes(&buf[..len]).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
