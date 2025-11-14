use openssh::{KnownHosts, Session, SessionBuilder};

pub async fn test_connection(hosts: Vec<String>) -> Result<(), String> {
    let proxy_jumps = if hosts.len() >= 2 {
        &hosts[1..hosts.len()-1]
    } else {
        &[] as &[String]
    };
    let remote = hosts.last().unwrap();

    let mut session = SessionBuilder::default()
        .jump_hosts(proxy_jumps)
        .connect(format!("{remote}")).await
            .map_err(|e| format!("connection failure: {e}"))?;

    Ok(())
}