// use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, ICMP};

// use crate::machine::Machine;

// use std::sync::Arc;


// pub trait Scanner {
//     fn scan(&self) -> Vec<Machine>;
// }

// pub struct PingScan {
//     address_list: Vec<String>,
// }

// impl Scanner for PingScan {
//     fn scan(&self) -> Vec<Machine> {
//         let client = Arc::new(Client::new(&Config::default())?);
//         let mut tasks = Vec::new();
//         let mut machines = Vec::new();

//         for ip in self.address_list {
//             client.pinger(host, ident)
//         }
//         machines
//     }
// }