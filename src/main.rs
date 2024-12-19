mod machine;
use machine::Machine;


use petgraph::prelude::*;

use petgraph::dot::{Config, Dot};

fn main() {
    let machine1 = Machine::new("192.168.0.1".to_string());
    let machine2 = Machine::new("192.168.0.2".to_string());

    let mut basic_graph = UnGraph::new_undirected();

    let node1 = basic_graph.add_node(machine1);
    let node2 = basic_graph.add_node(machine2);

    basic_graph.add_edge(node1, node2, 1);

    println!("{}", Dot::new(&basic_graph));
}
