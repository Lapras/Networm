mod machine;
mod dot_traits;
mod ipaddress;
mod writers;

use machine::Machine;

use dot_traits::DotNode;
use dot_traits::DotEdge;

use writers::FileWriter;
use writers::Writer;
use writers::StdWriter;

use petgraph::graph::Node;
use petgraph::prelude::*;

use petgraph::visit::IntoNodeReferences;

fn main() {
    let mut std_write: StdWriter = StdWriter::new();
    let mut file_write: FileWriter = FileWriter::new("../../testGraph.dot".to_string());

    let mut machine1 = Machine::new("Node1".to_string());
    let mut machine2 = Machine::new("Node2".to_string());
    let mut machine3 = Machine::new("Node3".to_string());
    let mut machine4 = Machine::new("Node4".to_string());
    let mut machine5 = Machine::new("Target".to_string());
    let mut machine6 = Machine::new("Router".to_string());

    machine1.add_address("192.168.10.2".to_string());
    machine2.add_address("192.168.10.3".to_string());
    machine3.add_address("192.168.10.4".to_string());
    machine4.add_address("192.168.10.5".to_string());
    machine5.add_address("192.168.10.6".to_string());
    machine6.add_address("192.168.10.7".to_string());
    machine1.add_address("192.168.11.2".to_string());
    machine2.add_address("192.168.11.3".to_string());
    machine6.add_address("192.168.11.7".to_string());

    let mut basic_graph = UnGraph::new_undirected();

    let node1 = basic_graph.add_node(machine1);
    let node2 = basic_graph.add_node(machine2);
    let node3 = basic_graph.add_node(machine3);
    let node4 = basic_graph.add_node(machine4);
    let node5 = basic_graph.add_node(machine5);
    let node6 = basic_graph.add_node(machine6);

    basic_graph.add_edge(node1, node2, ());
    basic_graph.add_edge(node2, node3, ());
    basic_graph.add_edge(node3, node4, ());
    basic_graph.add_edge(node5, node6, ());
    basic_graph.add_edge(node1, node6, ());


    print_graph(&mut file_write, basic_graph);
}

fn print_graph<W: Writer, N: DotNode>(writer: &mut W, graph: UnGraph<N, ()>) {
    let mut indent: i32 = 0;
    writer.writeln("graph Graph {".to_string(), indent);
    indent += 1;
    writer.writeln("node[shape=record, style=filled]".to_string(), indent);
    writer.writeln("splines=false".to_string(), indent);
    for(_node_index, node_data) in graph.node_references() {
            // Step 1: Print each node within  a cluster/all nodes in the graph
            writer.writeln(node_data.print_node(), indent);
    }

    for edge_index in graph.edge_indices() {
        let (source_index, target_index) = graph.edge_endpoints(edge_index).unwrap();
        let source: &N;
        let target: &N;
        let source_result = graph.node_weight(source_index);
        match source_result {
            Some(N) => source = N,
            None => continue,
        }
        let target_result = graph.node_weight(target_index);
        match target_result {
            Some(N) => target = N,
            None => continue,
        }

        writer.writeln(format!("{} -- {}", source.name(), target.name()), indent);
    }

    indent -= 1;
    writer.writeln("}".to_string(), indent);
}

