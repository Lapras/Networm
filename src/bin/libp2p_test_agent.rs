use std::error::Error;
use net_worm::agent::{init_tracing, build_swarm, run_swarm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();
    let swarm_instance = build_swarm()?;
    let remote = std::env::args().nth(1);
    run_swarm(swarm_instance, remote).await
}
