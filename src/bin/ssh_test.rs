use openssh::Session;
use tokio;
use net_worm::ssh::test_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let hosts = vec![
        "client".to_string(),
        "root@10.0.1.50".to_string(),
        "root@192.168.1.12".to_string(),
        "root@192.168.2.10".to_string(),
    ];

    // Start an SSH session (using default key-based authentication)
    match test_connection(hosts).await {
        Ok(()) => println!("Connection succesful"),
        Err(e) => println!("Connection failed {e}"),
    }

    Ok(())

}
