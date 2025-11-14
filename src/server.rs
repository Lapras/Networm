mod input;
mod commands;

use commands::{Command, AddCommand};

use crate::net_graph::NetGraph;
use crate::machine::Machine;

pub struct Server {
    network : NetGraph,
}

impl Server {
    pub fn new() -> Server {
        let graph = NetGraph::new();
        Server {
            network : graph
        }
    }

    pub fn server_loop(&mut self) {
        println!("N E T W O R M");
        loop {
            print!("> ");

            let input = input::parse_input(input::read_line());
            match input {
                Some(cli) => {
                    self.handle_command(cli.command)
                }
                None => {
                    println!("Invalid command")
                }
            }
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Add(add_cmd) => self.handle_add(add_cmd),
            Command::List => {
                self.list_machines();
            }
            _ => {
                println!("Unrecognized command")
            }
        }
    }

    fn handle_add(&mut self, add_cmd: AddCommand) {
       match add_cmd {
            AddCommand::Machine(commands::Machine { name, address }) => {
                self.add_machine(name, address)
            }
            AddCommand::Address(commands::Address { address }) => {
                println!("Add Address: {}", address);
                // handle adding address
            }
        }
    }
    
    fn add_machine(&mut self, name : String, address : Option<String>) {
        let mut machine = Machine::new(name.clone(), None);

        if let Some(ref addr) = address {
            machine.add_address(addr.clone());
        }

        self.network.add_node(machine);

        match &address {
            Some(addr) => println!("Added machine: {} with IP {}", name, addr),
            None => println!("Added machine: {}", name),
        }
    }

    fn list_machines(&mut self) {
        self.network.print_nodes();
    }
}
