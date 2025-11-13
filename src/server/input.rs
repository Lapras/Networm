use clap::Parser;

#[derive(Parser)]
struct RuntimeCli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, SubCommand)]
pub enum Command {
    Ping(PingCommand),
    Status(StatusCOmmand)
}

pub struct PingCommand {
    addr:String
}

pub fn handle_input(line: str) {
    
}