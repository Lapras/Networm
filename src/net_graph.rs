use petgraph::graph::Node;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use petgraph::algo::astar;

use crate::dot_traits::DotNode;
use crate::dot_traits::DotCluster;

use crate::writers::Writer;

use std::collections::HashSet;
use std::rc::Rc;

// NetGraph is essentially a wrapper for a Petgraph graph with some additional functionality
// Related to clustering nodes in the graph  
pub struct NetGraph {
    graph: UnGraph<Rc<dyn DotNode>, ()>,
    cluster_set: HashSet<Rc<DotCluster>>,
}

impl NetGraph {
    pub fn new() -> NetGraph {
        NetGraph {
            graph: UnGraph::new_undirected(),
            cluster_set: HashSet::new(),
        }
       
    }

    pub fn add_node<N: DotNode + 'static>(&mut self, node: N) -> NodeIndex {
        if let  Some(cluster) = node.get_cluster() {
            self.cluster_set.insert(cluster.clone());
        }

        self.graph.add_node(Rc::new(node))
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.graph.add_edge(source, target, ());
    }

    pub fn remove_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        if let Some(edge) = self.graph.find_edge(source, target) {
            self.graph.remove_edge(edge);
        }
    }

    pub  fn print_graph<W: Writer>(&self, writer: &mut W)  {
        let mut indent: i32 = 0;
        let mut cluster_num: i32 = 0;

        //HashSets so that we can cache the nodes/edges in clusters to filter them later
        let mut clustered_nodes: HashSet<NodeIndex> = HashSet::new();
        let mut clustered_edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

        writer.writeln("graph Graph {", indent);
        indent += 1;
        writer.writeln("node[shape=record, style=filled]", indent);
        writer.writeln("splines=false", indent);
        writer.writeln("", indent);

        for cluster in self.cluster_set.iter() {
            writer.writeln(&format!("subgraph cluster{} {{", cluster_num), indent);

            //print_cluster returns the nodes/edges that are contained within that cluster. The | operator adds those sets to our overall set
            let (new_nodes, new_edges) = self.print_cluster(writer, cluster, indent + 1);
            clustered_nodes = &clustered_nodes | &new_nodes;
            clustered_edges = &clustered_edges | &new_edges;

            writer.writeln("}", indent);
            writer.writeln("", indent);
            cluster_num += 1;
        }

        for(_,  node_data) in self.graph.node_references()
            .filter(|&(node_index, _)| !clustered_nodes.contains(&node_index)) {
            writer.writeln(&node_data.print_node(), indent);
        }

        writer.writeln("", indent);

        for edge in self.graph.edge_indices() {
            let (source_index, target_index) = self.graph.edge_endpoints(edge).unwrap();
            let source: &Rc<dyn DotNode>;
            let target: &Rc<dyn DotNode>;

            let source_result = self.graph.node_weight(source_index);
            match source_result {
                Some(result) => source = result,
                None => continue,
            }
            let target_result = self.graph.node_weight(target_index);
            match target_result {
                Some(result) => target = result,
                None => continue,
            }

            // We have to check both (source, target) and (target, source) as this is an undirected graph, failure to check results in double edges
            if !clustered_edges.contains(&(source_index, target_index))
                && !clustered_edges.contains(&(target_index, source_index)) {
                writer.writeln(&format!("{} -- {}", source.name().replace(" ", ""), target.name().replace(" ", "")), indent);
            }
        }

        indent -= 1;
        writer.writeln("}", indent);
    }

    fn print_cluster<W: Writer>(&self, writer: &mut W, cluster: &Rc<DotCluster>, indent: i32) -> (HashSet<NodeIndex>, HashSet<(NodeIndex, NodeIndex)>) {
        cluster.print_cluster_style(writer, indent);
        writer.writeln("", indent);

        let nodes: HashSet<NodeIndex> = self.find_nodes_in_cluster(cluster);
        let mut edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

        for node in nodes.iter() {
            if let Some(node_reference) = self.graph.node_weight(*node) {
                writer.writeln(&node_reference.print_node(), indent);
            }
        }
        writer.writeln("", indent);

        for node in nodes.iter() {
            for edge in self.graph.edges(*node) {
                let source = &self.graph[edge.source()];
                let target = &self.graph[edge.target()];
                if source.get_cluster() == target.get_cluster() 
                && !edges.contains(&(edge.source(), edge.target()))
                && !edges.contains(&(edge.target(), edge.source())) {
                    edges.insert((edge.source(), edge.target()));
                    writer.writeln(&format!("{} -- {}", source.name().replace(" ", ""), target.name().replace(" ", "")), indent);
                }
            }
        }

        (nodes, edges)
    }

    fn find_nodes_in_cluster(&self, cluster: &Rc<DotCluster>) -> HashSet<NodeIndex> {
        let mut nodes: HashSet<NodeIndex> = HashSet::new();
            for(node_index, node_data) in self.graph.node_references() {
                if let Some(node_cluster) = node_data.get_cluster() {
                    if cluster == node_cluster {
                        nodes.insert(node_index);
                    }
                }
            }
        nodes
    }

    pub fn find_node(&self, name: &str) -> Option<NodeIndex> {
        for (node_index, node_data) in self.graph.node_references() {
            if node_data.name() == name {
                return Some(node_index);
            }
        }
        None
    }

    pub fn find_node_data(&self, name : &str) -> Option<&Rc<dyn DotNode>> {
        if let Some(idx) = self.find_node(name) {
            return self.get_node_data(idx)
        } else  {
            return None
        }
    }


    pub fn find_pair(
        &self,
        name1: &str,
        name2: &str,
        ) -> Result<(NodeIndex, NodeIndex), String> {
        let maybe_node1 = self.find_node(name1);
        let maybe_node2 = self.find_node(name2);

        match (maybe_node1, maybe_node2) {
            (Some(n1), Some(n2)) => Ok((n1, n2)),
            (None, Some(_)) => Err(format!("Machine '{}' not found", name1)),
            (Some(_), None) => Err(format!("Machine '{}' not found", name2)),
            (None, None) => Err(format!("Machines '{}' and '{}' not found", name1, name2)),
        }
    }

    pub fn find_path(&self, name1: &str, name2: &str) {
        if let Ok((src, dest)) = self.find_pair(name1, name2) {
            let result = astar(&self.graph,
                src,
              |node| node == dest,
            |_| 1,
        |_| 0);
            match result {
                Some((cost, path)) => {
                    println!("{cost} hops");
                    for node in path {
                        println!(" - {}", self.graph[node].name())
                    }
                }
                None => println!("Failed to find path")
            }
        } else {
            println!("Failed to find pair");
        }
    }


    fn get_node_data(&self, idx : NodeIndex) -> Option<&Rc<dyn DotNode>> {
        return self.graph.node_weight(idx);
    }

    pub fn print_nodes(&self) {
        for (_idx, node_rc) in self.graph.node_references() {
            // node_rc is an Rc<dyn DotNode>
            println!(
                "Machine: {}",
                node_rc.name(),
            );
        }
    }

}