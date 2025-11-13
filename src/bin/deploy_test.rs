use openssh::Session;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use openssh_sftp_client;

const CHUNK_SIZE: usize = 16 * 1024;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect via SSH & open SFTP. You may need to use the companion `openssh` crate
    // for the SSH session, then pass something to the SFTP client.

    let ssh_session = Session::connect(
        "root@10.0.1.50",
        openssh::KnownHosts::Accept
    ).await?;

    let mut sftp = openssh_sftp_client::Sftp::from_session(ssh_session,openssh_sftp_client::SftpOptions::default()).await?;
    
    let mut local_file = File::open("target/debug/agent").await?;
    let metadata = local_file.metadata().await?;
    let total_size = metadata.len();

    let mut remote = sftp.create("/tmp/sftp_test").await?;

    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut uploaded: u64 = 0;

    loop {
        // Read chunk
        let n = local_file.read(&mut buffer).await?;
        if n == 0 {
            break; // EOF
        }

        // Write chunk
        remote.write_all(&buffer[..n]).await?;

        // Update progress
        uploaded += n as u64;
        let percent = (uploaded as f64 / total_size as f64) * 100.0;
        println!("Uploaded {}/{} bytes ({:.1}%)", uploaded, total_size, percent);
    }

    
    println!("Successfully uploaded file!");
    Ok(())
}
