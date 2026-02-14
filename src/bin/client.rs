use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::fs::{File, create_dir_all};
use tokio::io;
use std::{io::stdin, path::PathBuf};
use dirs::home_dir;


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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");

    create_dir_all(&sync_dir).await?;
    
    let mut stream = TcpStream::connect("127.0.0.1:2828").await?;
 
    println!("Enter the name of the file");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("error reading user input");

    let file_path = sync_dir.join(file_name.trim());

    let mut write_path = sync_dir.into_os_string().into_string().unwrap();
    println!("Default path is: {} -- Do you want to modify it?
        Press enter to skip, write the new path to modify.",
        file_path.display());
    
    let mut modified_path = String::from("/");
    stdin().read_line(&mut modified_path).expect("error reading user input");

    if !modified_path.trim().is_empty() {
        write_path += &modified_path; 
    } 

    send_file(&mut stream, file_path, write_path).await?;
    Ok(())
}
