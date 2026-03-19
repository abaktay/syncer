use std::path::PathBuf;
use std::net::IpAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::fs::{File, create_dir_all};
use tokio::io::{self, AsyncBufReadExt};
use dirs::home_dir;
use mdns_sd::{ServiceDaemon, ServiceInfo};

fn get_local_ip() -> anyhow::Result<IpAddr> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?.ip())
}

async fn read_file(stream: &mut TcpStream,  dir:  PathBuf) -> anyhow::Result<()> {
    let mut reader = io::BufReader::new(stream);
    // name of the file -- with the complete path
    // \n acts as a separator
    // should receive the file path after /home/user_name/sync/
    let mut file_name = String::new();
    reader.read_line(&mut file_name).await?;
    println!("File name {}", file_name.trim());
    
    let write_path = dir.join(&file_name.trim_end());
    println!("Writing to {}", write_path.display());

    let mut file = File::create(write_path).await?;

    let bytes = io::copy(&mut reader, &mut file).await?;
   
    println!("Received {} bytes", bytes);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let daemon = ServiceDaemon::new().expect("Failed to create daemon");
    
    let my_ip = get_local_ip()?;
    let hostname = "receiver.local.";
    let service_type = "_p2pfile._tcp.local.";
    let instance_name = "file-receiver";
    let port = 2828;
    
    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        &hostname,
        my_ip.to_string().as_str(),
        port,
        None,
    )?;
    
    daemon.register(service_info)?;
    println!("Registered mDNS service at {}:{}", my_ip, port);
    // sync directory
    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");
    create_dir_all(&sync_dir).await?;
    
    let listener = TcpListener::bind("0.0.0.0:2828").await?;

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        let sync_dir_clone = sync_dir.clone();
        tokio::spawn(async move {
            if let Err(e) = read_file(&mut stream, sync_dir_clone).await {
                eprintln!("Error receiving file: {}", e);
            }
        });
    }

    Ok(())
}
