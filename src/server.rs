use std::{error::Error};
use futures::prelude::*;
use libp2p::{noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr, Swarm};
use tokio::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::Duration;
use tracing_subscriber::EnvFilter;

/// Initialize tracing/logging
pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

/// Spawn a task that reads stdin lines and sends them through a channel
pub fn spawn_stdin_task() -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if tx.send(line).is_err() {
                break; // receiver dropped, exit task
            }
        }
    });

    rx
}

/// Build a ping-enabled libp2p swarm
pub fn build_swarm() -> Result<Swarm<ping::Behaviour>, Box<dyn Error>> {
    let swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    Ok(swarm)
}

/// Run the main event loop: swarm events + user input
pub async fn run_interactive(mut swarm: Swarm<ping::Behaviour>, mut rx: mpsc::UnboundedReceiver<String>, listen_addr: &str) -> Result<(), Box<dyn Error>> {
    swarm.listen_on(listen_addr.parse()?)?;

    loop {
        tokio::select! {
            swarm_event = swarm.select_next_some() => {
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => println!("Swarm event: {event:?}"),
                    _ => {}
                }
            }
            Some(line) = rx.recv() => {
                println!("User typed: {}", line);
                // TODO: send over libp2p or handle commands
            }
        }
    }
}
