use openssh::{KnownHosts, Session, SessionBuilder};
use tokio::time::{timeout, Duration};

pub async fn test_connection(hosts: Vec<String>) -> Result<(), String> {
    let proxy_jumps = if hosts.len() >= 2 {
        &hosts[1..hosts.len()-1]
    } else {
        &[] as &[String]
    };
    let remote = hosts.last().unwrap();

    let timeout_duration = Duration::from_secs(5);

    let session_result = timeout(timeout_duration, async {
        SessionBuilder::default()
            .jump_hosts(proxy_jumps)
            .connect(format!("{remote}"))
            .await
        })
        .await
        .map_err(|_| "connection timed out".to_string())?;

    Ok(())
}