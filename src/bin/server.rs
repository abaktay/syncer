use tokio::net::TcpListener;
 use tokio::net::TcpStream;
use tokio::fs::{File, create_dir_all};
use tokio::io::{self, AsyncBufReadExt};
use dirs::home_dir;

async fn read_file(stream: &mut TcpStream) -> anyhow::Result<()> {

    let mut reader = io::BufReader::new(stream);

    // name of the file -- with the complete path
    let mut file_name = String::new();
    reader.read_line(&mut file_name).await?;

    println!("Writing to {}", file_name);

    let mut file = File::create(file_name.trim()).await?;

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

    read_file(&mut stream).await?;

    Ok(())
}
