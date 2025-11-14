use clap::{Parser};
use std::io::{self};

use crate::server::commands;
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InputParser {
    #[command(subcommand)]
    pub command: commands::Command,
}


pub fn parse_input(line: String) -> Option<InputParser> {
    let args = line.split_whitespace();
    let args = std::iter::once("networm").chain(args);

    match InputParser::try_parse_from(args) {
        Ok(cli) => {
            Some(cli)
        },
        Err(_err) => {
            None
        }
    }
}

pub fn read_line() -> String {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            // println!("Read line: {buffer}")
        },
        Err(_e) => {
            // println!("Error reading input: {e}")
        }
    }

    buffer
}