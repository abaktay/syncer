use std::path::PathBuf;
use tokio::io::{self, AsyncReadExt};

use tokio::fs::File;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn read_file(stream: &mut TcpStream, dir: PathBuf) -> anyhow::Result<()> {
    // let mut reader = io::BufReader::new(stream);
    // name of the file -- with the complete path
    // \n acts as a separator
    // should receive the file path after /home/user_name/sync/

    // name_len -> name -> file_len -> file
    let name_len = stream.read_u32().await?;
    let mut file_name = vec![0u8; name_len as usize];
    stream.read_exact(&mut file_name).await?;
    let file_size = stream.read_u64().await?;

    let file_name = String::from_utf8(file_name)?;
    println!("Received path: {}", file_name);

    let mut file = File::create(dir.join(file_name)).await?;

    let mut limited = stream.take(file_size);
    tokio::io::copy(&mut limited, &mut file).await?;

    Ok(())
}

// stream: Tcp Stream
// file_path: path of the file to send
// order: name_len -> name -> file_len -> file
pub async fn send_file(stream: &mut TcpStream, path: &PathBuf) -> anyhow::Result<()> {
    // includes only the file name
    // /home/foo/balls/garfield.txt -> garfield.txt

    let file_name = path.file_name().unwrap();

    let mut file = File::open(path).await?;
    let metadata = file.metadata().await?;
    println!("size is {}", metadata.len());
    stream.write_u32(file_name.len() as u32).await?;
    stream
        .write_all(file_name.to_str().expect("file name got messed").as_bytes())
        .await?;
    stream.write_u64(metadata.len()).await?;

    // sends the file
    let bytes = io::copy(&mut file, stream).await?;
    stream.shutdown().await?;
    println!("Sent {} bytes", bytes);
    Ok(())
}
