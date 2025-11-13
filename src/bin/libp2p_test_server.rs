use std::error::Error;
use net_worm::server;
use net_worm::tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing::init_tracing();
    let rx = server::spawn_stdin_task();
    let swarm = server::build_swarm()?;

    server::run_interactive(swarm, rx, "/ip4/0.0.0.0/tcp/4001").await
}
