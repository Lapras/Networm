use std::fmt::{self, format};
use crate::dot_traits::DotNode;
use crate::ipaddress::{self, IpAddress};
pub struct Machine {
    name: String,
    address_list: Vec<String>,
}

impl Machine {
    pub fn new(name: String) -> Machine {
        Machine {
            name: name,
            address_list: Vec::new(),
        }
    }

    pub fn add_address(&mut self, address: String) {
        self.address_list.push(address);
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_node())
    }
}

impl DotNode for Machine {
    fn print_node(&self) -> String {
        let mut out: String = format!("{} [label=\"{{ {}", self.name, self.name);
        for ipaddress in self.address_list.iter() {
            out.push_str(format!(" | {{ IP | {} }}", ipaddress).as_str());
        }
        out.push_str("}\"];");
        out
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

