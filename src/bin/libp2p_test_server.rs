use std::{error::Error};
use futures::prelude::*;
use libp2p::{noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr};
use tracing_subscriber::EnvFilter;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    // Channel for stdin lines
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Spawn a separate task to handle stdin
    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if tx.send(line).is_err() {
                break; // receiver dropped, exit task
            }
        }
    });

    // Build libp2p swarm
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse()?)?;

    // Main event loop
    loop {
        tokio::select! {
            swarm_event = swarm.select_next_some() => {
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => println!("Swarm event: {event:?}"),
                    _ => {}
                }
            }

            // Receive user input from stdin task
            Some(line) = rx.recv() => {
                println!("User typed: {}", line);
                // Here you could send it over libp2p or handle commands
            }
        }
    }
}
