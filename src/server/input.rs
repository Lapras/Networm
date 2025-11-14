use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InputParser {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(Add),
    List,
}

#[derive(Debug)]
struct Add {
    name : String
}

pub fn parse_input(line: String) {
    let args = line.split_whitespace();
    let args = std::iter::once("agent").chain(args);

    match InputParser::try_parse_from(args) {
    Ok(cli) => println!("Parsed successfully!"),
    Err(err) => println!("Failed: {}", err),
    }
}

pub fn read_line() -> String {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            println!("Read line: {buffer}")
        },
        Err(e) => {
            println!("Error reading input: {e}")
        }
    }

    buffer
}