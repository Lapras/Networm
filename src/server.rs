mod input;

use std::io::{self, Write};
use server::input;

pub fn server_loop() {
    println!("N E T W O R M");
    loop {
        print!("> ");

        let command = input::parse_input(input::read_line());

        
    }
}


