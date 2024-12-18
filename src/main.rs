mod machine;
use machine::Machine;


use petgraph::Graph;
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

fn main() {
    let machine1 = Machine::new("192.168.0.1".to_string());
    let machine2 = Machine::new("192.168.0.2".to_string());

    let mut basic_graph = Graph::<&Machine, &Machine>::new();

    let node1 = basic_graph.add_node(&machine1);
    let node2 = basic_graph.add_node(&machine2);

    basic_graph.extend_with_edges(&[(node1, node2)]);
}
