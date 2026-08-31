use dirs::home_dir;
use std::time::Duration;
use sync::discovery::*;
use sync::id::Id;
use sync::transfer::*;
use tokio::fs::create_dir_all;
use tokio::net::{TcpListener, UdpSocket};

const DATA_PORT: u16 = 8888;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_ip = get_local_ip()?;
    // let my_ip = std::net::Ipv4Addr::LOCALHOST; // localhost for debugging

    let listener = TcpListener::bind(format!("0.0.0.0:{}", DATA_PORT)).await?;
    let receiver_id = Id::new(my_ip, String::from("mahmut"), DATA_PORT);
    tokio::spawn(async move {
        let disc_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        disc_socket.set_broadcast(true).unwrap();

        let timeout = Duration::from_secs(15);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Err(e) = peer_broadcast(&disc_socket, &receiver_id).await {
                eprintln!("Broadcast error: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // sync directory
    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");
    create_dir_all(&sync_dir).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        let sync_dir = sync_dir.clone();
        tokio::spawn(async move {
            if let Err(e) = read_file(&mut stream, sync_dir).await {
                eprintln!("Error receiving file: {}", e);
            }
        });
    }
}
