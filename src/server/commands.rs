use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InputArgs {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {
    List,
    #[clap(subcommand)]
    Add(AddCommand),
    Connect(Connect),
    Disconnect(Connect),
    Path(Path),
    SetLocal(SetLocal),
    Test(Test),
    PrintGraph(File),
    ReadInput(File),
    Exit,
}

#[derive(Debug, Subcommand)]
pub enum AddCommand {
    Machine(Machine),
    Address(Address),
}

#[derive(Debug, Args)]
pub struct Test {
    #[clap(short, long)]
    pub recurisve: bool,
    pub source: String,
    pub dest: String,
}

#[derive(Debug, Args)]
pub struct Machine {
    pub name: String,
    pub address: Option<String>,
}

#[derive(Debug, Args)]
pub struct Address {
    pub address : String,
}

#[derive(Debug, Args)]
pub struct Connect {
    pub name1 : String,
    pub name2 : String
}

#[derive(Debug, Args)]
pub struct SetLocal {
    pub name : String
}

#[derive(Debug, Args)]
pub struct Path {
    pub source : String,
    pub dest : String,
}

#[derive(Debug, Args)]
pub struct File {
    pub name : String,
}
