use dirs::home_dir;
use std::io::stdin;
use sync::{discovery::listen_for_peers, transfer::send_file};
use tokio::{
    fs::create_dir_all,
    net::{TcpStream, UdpSocket},
};

const DISCOVERY_PORT: u16 = 2828;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT)).await?;

    let peer_id = listen_for_peers(&socket).await?;

    let peer_addr = format!("{}:{}", peer_id.ip, peer_id.port);
    let mut stream = TcpStream::connect(peer_addr).await?;

    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");

    create_dir_all(&sync_dir).await?;

    loop {
        println!("Enter the name of the file");
        let mut file_name = String::new();
        stdin()
            .read_line(&mut file_name)
            .expect("error reading user input");

        let file_path = sync_dir.join(file_name.trim());

        send_file(&mut stream, &file_path).await?;
    }
}
