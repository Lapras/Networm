use openssh::Session;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change this to your actual SSH target
    let target = "root@10.0.1.50";

    // Start an SSH session (using default key-based authentication)
    let session = Session::connect(target, openssh::KnownHosts::Strict).await?;
    println!("✅ Connected to {}", target);

    // Run the 'ls' command on the remote host
    let output = session.command("ls").output().await?;

    // Print stdout and stderr from the command
    println!("--- STDOUT ---\n{}", String::from_utf8_lossy(&output.stdout));
    println!("--- STDERR ---\n{}", String::from_utf8_lossy(&output.stderr));

    // Close the session cleanly
    session.close().await?;
    println!("✅ Session closed.");

    Ok(())
}
