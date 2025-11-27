use openssh::{KnownHosts, Session, SessionBuilder};
use tokio::time::{timeout, Duration};
use std::process::Command;

pub async fn test_connection(hosts: Vec<String>) -> Result<(), String> {
    let proxy_jumps = if hosts.len() >= 2 {
        &hosts[1..hosts.len()-1]
    } else {
        &[] as &[String]
    };
    let remote = hosts.last().unwrap();

    let timeout_duration = Duration::from_secs(10);

    let _session_result = timeout(timeout_duration, async {
        SessionBuilder::default()
            .jump_hosts(proxy_jumps)
            .connect(format!("{remote}"))
            .await
        })
        .await
        .map_err(|_| "connection timed out".to_string())?
        .map_err(|e| format!("SSH_connection failed: {e}"))?;

    Ok(())
}