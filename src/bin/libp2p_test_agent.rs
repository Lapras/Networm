use std::error::Error;
use net_worm::agent::{build_swarm, run_swarm};
use net_worm::tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing::init_tracing();
    let swarm_instance = build_swarm()?;
    let remote = std::env::args().nth(1);
    run_swarm(swarm_instance).await
}
