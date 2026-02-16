use std::path::PathBuf;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::fs::{File, create_dir_all};
use tokio::io::{self, AsyncBufReadExt};
use dirs::home_dir;

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
    // sync directory
    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");
    create_dir_all(&sync_dir).await?;

    let listener = TcpListener::bind("127.0.0.1:2828").await?;
     
    let (mut stream, addr) = listener.accept().await?;
    println!("Connection from {}", addr);

    read_file(&mut stream, sync_dir).await?;

    Ok(())
}
