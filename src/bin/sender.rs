use tokio::{io::AsyncWriteExt, net::{UdpSocket,TcpStream}};
use tokio::fs::{File, create_dir_all};
use tokio::io;
use std::{io::stdin, path::PathBuf};
use dirs::home_dir;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;

// stream: Tcp Stream
// file_path: path of the file to send
// write_addr: address to write 
async fn send_file(stream: &mut TcpStream, file_path: PathBuf, write_path: String) -> anyhow::Result<()> {
    // sends the path
    stream.write_all(write_path.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    println!("size is {}", metadata.len());
    
    // sends the file
    let bytes = io::copy(&mut file, stream).await?;
    stream.shutdown().await?;
    println!("Sent {} bytes", bytes);
    Ok(())
}

fn discover_receiver() -> anyhow::Result<String> {
    let daemon = ServiceDaemon::new()?;
    let service_type = "_p2pfile._tcp.local.";
    
    let receiver = daemon.browse(service_type)?;
    
    println!("Searching for receiver");
    
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(500)) {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("Found service: {}", info.get_fullname());

                    if let Some(ip) = info.get_addresses().iter().next() {
                        let addr = format!("{}:{}", ip, info.get_port());
                        println!("Receiver address: {}", addr);
                        daemon.shutdown()?;
                        return Ok(addr);
                    }
                }

                other => {
                    println!("Event: {:?}", other);
                }
            }
        }
    }
    
    daemon.shutdown()?;
    Err(anyhow::anyhow!("No receiver found"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let receiver_addr = discover_receiver()?;

    let mut stream = TcpStream::connect(&receiver_addr).await?;

    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");

    create_dir_all(&sync_dir).await?;
  
    println!("Enter the name of the file");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("error reading user input");

    let file_path = sync_dir.join(file_name.trim());

    println!("Default path is: {} -- Do you want to modify it?
        Press enter to skip, write the new path to modify.",
        file_path.display());
    
    let mut modified_path = String::new();
    stdin().read_line(&mut modified_path).expect("error reading user input");

    if !modified_path.trim().is_empty() {
        file_name = modified_path; 
    }

    send_file(&mut stream, file_path, file_name).await?;
    Ok(())
}
