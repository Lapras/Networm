use std::fmt;
use std::rc::Rc;
use crate::dot_traits::{DotNode, DotCluster};


pub struct Machine {
    name: String,
    address_list: Vec<String>,
    cluster: Option<Rc<DotCluster>>,
}

impl Machine {
    pub fn new(name: String, cluster: Option<&Rc<DotCluster>>) -> Machine {
        let cluster = match cluster {
            Some(c) => Some(c.clone()),
            None => None,
        };

        Machine {
            name: name,
            address_list: Vec::new(),
            cluster: cluster,
        }
    }

    pub fn add_address(&mut self, address: String) {
        self.address_list.push(address);
    }

    pub fn set_cluster(&mut self, cluster: Rc<DotCluster>) {
        self.cluster = Some(cluster);
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_node())
    }
}

impl DotNode for Machine {
    fn print_node(&self) -> String {
        let mut out: String = format!("{} [label=\"{{ {}", self.name.replace(" ", ""), self.name);
        for ipaddress in self.address_list.iter() {
            out.push_str(format!(" | {{ IP | {} }}", ipaddress).as_str());
        }
        out.push_str("}\"];");
        out
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_cluster(&self) -> &Option<Rc<DotCluster>>  {
        &self.cluster
    }
    fn address(&self) -> Vec<String> {
        self.address_list.clone()
    }
}

