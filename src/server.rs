mod input;
mod commands;

use commands::{Command, AddCommand, Connect, SetLocal, Path};

use crate::net_graph::{self, NetGraph};
use crate::dot_traits::DotNode;
use crate::machine::Machine;
use crate::ssh::test_connection;
use tokio;

use crate::writers::FileWriter;

use std::fs::File;
use std::rc::Rc;
use std::io::BufRead;

use std::io::{BufReader, Write, stdout};

pub struct Server {
    network : NetGraph,
    local_machine : Option<Rc<dyn DotNode>>
}

impl Server {
    pub fn new() -> Server {
        let graph = NetGraph::new();
        Server {
            network : graph,
            local_machine : None
        }
    }

    pub fn server_loop(&mut self) {
        println!("N E T W O R M");
        loop {
            let mut cont = true;
            print!("> ");
            stdout().flush();

            let input = input::parse_input(input::read_line());
            match input {
                Some(cli) => {
                    cont = self.handle_command(cli.command);
                }
                None => {
                    println!("Invalid command")
                }
            }
            if !cont {
                break
            }
        }
    }

    fn handle_command(&mut self, command: Command) -> bool {
        match command {
            Command::Add(add_cmd) => self.handle_add(add_cmd),
            Command::List => {
                self.list_machines();
            }
            Command::Exit => {
                return false
            }
            Command::Connect(cmd) => {
                self.connect_machines(cmd);
            }
            Command::Disconnect(cmd) => {
                self.disconnect_machines(cmd);
            }
            Command::Path(cmd) => {
                self.print_path(cmd.source, cmd.dest);
            }
            Command::SetLocal(cmd) => {
                self.set_local_machine(cmd.name);
            }
            Command::Test(cmd) => {
                println!("Testing a connection");
                self.test_connect(&cmd.source, &cmd.dest);
            }
            Command::PrintGraph(cmd) => {
                let filename = format!("{}.dot",cmd.name);
                let mut file_write: FileWriter = FileWriter::new(&filename);
                println!("Writing graph to file {}", cmd.name.to_string());
                self.network.print_graph(&mut file_write);
            }
            Command::ReadInput(cmd) => {
                if let Ok(file) = File::open(&cmd.name) {
                    let reader = BufReader::new(file);
                    for line_result in reader.lines() {
                        match line_result {
                            Ok(line) => {
                                let input = input::parse_input(line);
                                match input {
                                    Some(cli) => {
                                        self.handle_command(cli.command);
                                    }
                                    None => {
                                        println!("Invalid command")
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error reading line: {e}"),
                        }
                    }
                } else {
                    println!("Could not find file {}", cmd.name.to_string())
                }

            }
            _ => {
                println!("Unrecognized command")
            }
        }
        return true
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

    fn connect_machines(&mut self, connect : Connect) {
        let name1 = connect.name1.clone();
        let name2 = connect.name2.clone();

        let maybe_node1 = self.network.find_node(&connect.name1);
        let maybe_node2 = self.network.find_node(&connect.name2);

        match (maybe_node1, maybe_node2) {
            (Some(node1), Some(node2)) => {
                self.network.add_edge(node1, node2);
                println!("Connected {} and {}", name1, name2);
            }
            (None, Some(_)) => {
                println!("Machine '{}' not found", name1);
            }
            (Some(_), None) => {
                println!("Machine '{}' not found", name2);
            }
            (None, None) => {
                println!("Machines '{}' and '{}' not found", name1, name2);
            }
        }
    }

    fn disconnect_machines(&mut self, connect : Connect) {
        let name1 = connect.name1.clone();
        let name2 = connect.name2.clone();

        let maybe_node1 = self.network.find_node(&connect.name1);
        let maybe_node2 = self.network.find_node(&connect.name2);

        match (maybe_node1, maybe_node2) {
            (Some(node1), Some(node2)) => {
                self.network.remove_edge(node1, node2);
                println!("Disconnected {} and {}", name1, name2);
            }
            (None, Some(_)) => {
                println!("Machine '{}' not found", name1);
            }
            (Some(_), None) => {
                println!("Machine '{}' not found", name2);
            }
            (None, None) => {
                println!("Machines '{}' and '{}' not found", name1, name2);
            }
        }
    }

    fn print_path(&mut self, src_name : String, dst_name : String) {
        self.network.print_path(&src_name, &dst_name)
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

    fn set_local_machine(&mut self, name : String) {
        match self.network.find_node_data(&name) {
            Some(node) => {
                self.local_machine =  Some(node.clone());
                println!("Set machine {name}  as local")
            }
            None => println!("Could not fine machine with name {name}"),
        };
    }

    fn test_connect(&mut self, source : &str, dest : &str) {
        println!("Testing connection using path:");
        self.print_path(source.to_string(), dest.to_string());
        match self.network.find_path(&source, &dest) {
            Ok(path) => {
                let addresses = path.iter()
                    .filter_map(|node| node.address().get(0).cloned())
                    .map(|addr| format!("root@{}", addr))
                    .collect();

                 let result = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(test_connection(addresses));
                match result {
                    Ok(()) => println!("Connection succesful"),
                    Err(e) => println!("Connection failed {e}"),
                    }
                }
            Err(e) => {
                println!("Error finding path {e}");
            }
        }
    }

       

    fn list_machines(&mut self) {
        self.network.print_nodes();
        if let Some(machine) = &self.local_machine {
            println!("Local machine: {}", machine.name());
        }
    }
}
