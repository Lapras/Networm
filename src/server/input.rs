use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InputParser {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Ping,
    Status,
}

// pub struct PingCommand {
//     addr:String
// }

// pub struct StatusCommand {
//     addr:String
// }

pub fn handle_input(line: String) {
      let args = line.split_whitespace();
      let args = std::iter::once("agent").chain(args);

      match InputParser::try_parse_from(args) {
        Ok(cli) => println!("Parsed successfully!"),
        Err(err) => println!("Failed: {}", err),
      }
}