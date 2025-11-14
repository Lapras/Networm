mod input;
mod commands;

use commands::Command;

use crate::server::input::InputParser;


pub fn server_loop() {
    println!("N E T W O R M");
    loop {
        print!("> ");

        let input = input::parse_input(input::read_line());

        match input {
            Some(command) => {

            }
            None => {
                println!("Invalid command")
            }
        }
    }
}

fn handle_command(command: InputParser) {
    match command {
        Command::List => {
            println!("Handling the list command");
        }
        _ => {
            println!("Unrecognized Command")
        }
    }
}