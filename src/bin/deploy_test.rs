use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let local_file = "./target/debug/agent";
    let remote_user = "root";
    let remote_host = "10.0.1.50";
    let remote_path = "/tmp/my_binary";

    Command::new("scp")
        .arg(local_file)
        .arg(format!("{}@{}:{}", remote_user, remote_host, remote_path))
        .status()?;

    println!("File transferred via scp.");

    Ok(())
}
