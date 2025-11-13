use std::{error::Error, time::Duration};
use futures::prelude::*;
use libp2p::{noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr, Swarm};
use tracing_subscriber::EnvFilter;

mod input;

use input::Cli;
use clap::Parser;

pub struct Agent {

}

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

pub async fn run_swarm(mut swarm: Swarm<ping::Behaviour>) -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let addr = args.remote;
    let addr = format!("/ip4/{}/tcp/4001", addr);
    let remote: Multiaddr = addr.parse()?;
    swarm.dial(remote)?;
    println!("Dialed {addr}");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}