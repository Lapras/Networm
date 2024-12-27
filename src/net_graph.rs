use petgraph::graph::Node;
use petgraph::graph::NodeWeightsMut;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;

use crate::dot_traits::DotNode;
use crate::dot_traits::Cluster;

use crate::writers::FileWriter;
use crate::writers::MultiWriter;
use crate::writers::Writer;
use crate::writers::StdWriter;

use std::collections::HashSet;
use std::rc::Rc;

// NetGraph is essentially a wrapper for a Petgraph graph with some additional functionality
// Related to clustering nodes in the graph  
pub struct NetGraph {
    graph: UnGraph<Rc<dyn DotNode>, ()>,
    cluster_set: HashSet<Rc<Cluster>>,
}

impl NetGraph {
    pub fn new() -> NetGraph {
        NetGraph {
            graph: UnGraph::new_undirected(),
            cluster_set: HashSet::new(),
        }
       
    }

    pub fn add_node<N: DotNode + 'static>(&mut self, node: N) {
        if let  Some(cluster) = node.get_cluster() {
            self.cluster_set.insert(cluster.clone());
        }

        self.graph.add_node(Rc::new(node));
    }

    pub  fn print_graph<W: Writer>(&self, writer: &mut W)  {
        let mut indent: i32 = 0;
        let mut cluster_num: i32 = 0;
        writer.writeln("graph Graph {", indent);
        indent += 1;
        writer.writeln("node[shape=record, style=filled]", indent);
        writer.writeln("splines=false", indent);

        for cluster in self.cluster_set.iter() {
            writer.writeln(&format!("subgraph cluster{} {{", cluster_num), indent);
            self.print_cluster(writer, cluster, indent + 1);
            writer.writeln("}}", indent);
        }
    }

    fn print_cluster<W: Writer>(&self, writer: &mut W, cluster: &Rc<Cluster>, indent: i32) {
        //Print beginning cluster info here

        let nodes: HashSet<NodeIndex> = self.find_nodes_in_cluster(cluster);
        let mut edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

        for node in nodes.iter() {
            for edge in self.graph.edges(*node) {
                if self.graph[edge.target()].get_cluster() == self.graph[edge.source()].get_cluster() 
                && !edges.contains(&(edge.source(), edge.target())) {
                    edges.insert((edge.source(), edge.target()));
                    //print here
                }
            }
        }
    }

    fn find_nodes_in_cluster(&self, cluster: &Rc<Cluster>) -> HashSet<NodeIndex> {
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
}