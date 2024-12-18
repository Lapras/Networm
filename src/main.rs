mod machine;
use machine::Machine;


use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

fn main() {
    let machine1 = Machine::new("192.168.0.1".to_string());
    let machine2 = Machine::new("192.168.0.2".to_string());
    let g = UnGraph::<machine::Machine, ()>::from_edges(&[(&machine1, &machine2)]);
}
