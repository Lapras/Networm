use clap::{Parser, Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum Command {
    List
}