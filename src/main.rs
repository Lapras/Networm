mod machine;
mod dot_traits;
mod ipaddress;
mod writers;
mod net_graph;

use std::rc::Rc;

use dot_traits::DotCluster;
use machine::Machine;

use net_graph::NetGraph;
use writers::{FileWriter, MultiWriter, StdWriter};

fn main() {
    let mut net_graph: NetGraph = NetGraph::new();

    let std_write: StdWriter = StdWriter::new();
    let file_write: FileWriter = FileWriter::new("testGraph.dot");
    let mut multi_write: MultiWriter = MultiWriter::new();

    multi_write.add_writer(Box::new(std_write));
    multi_write.add_writer(Box::new(file_write));

    let blue_space: Rc<DotCluster> = Rc::new(DotCluster::new("Blue space", "blue"));
    let red_space: Rc<DotCluster> = Rc::new(DotCluster::new("Red space", "red"));
    let gray_space: Rc<DotCluster> = Rc::new(DotCluster::new("gray space", "gray"));

    let mut workstation1 = Machine::new("Workstation 1".to_string(), Some(&blue_space));
    workstation1.add_address("192.168.2.3".to_string());
    let workstation1_node = net_graph.add_node(workstation1);

    let mut workstation2 = Machine::new("Workstation 2".to_string(), Some(&blue_space));
    workstation2.add_address("192.168.2.4".to_string());
    let workstation2_node = net_graph.add_node(workstation2);

    let mut workstation3 = Machine::new("Workstation 3".to_string(), Some(&blue_space));
    workstation3.add_address("192.168.2.5".to_string());
    let workstation3_node = net_graph.add_node(workstation3);

    let mut switch = Machine::new("Switch".to_string(), Some(&blue_space));
    switch.add_address("192.168.2.2".to_string());
    let switch_node = net_graph.add_node(switch);

    let mut router = Machine::new("Router".to_string(), Some(&blue_space));
    router.add_address("192.168.2.1".to_string());
    router.add_address("192.0.1.7".to_string());
    let router_node = net_graph.add_node(router);

    let mut proxy1 = Machine::new("Proxy 1".to_string(), Some(&gray_space));
    proxy1.add_address("192.0.2.3".to_string());
    let proxy1_node = net_graph.add_node(proxy1);
    
    let mut proxy2 = Machine::new("Proxy 2".to_string(), Some(&gray_space));
    proxy2.add_address("192.0.23.3".to_string());
    let proxy2_node = net_graph.add_node(proxy2);

    let mut proxy3 = Machine::new("Proxy 3".to_string(), Some(&gray_space));
    proxy3.add_address("192.0.156.4".to_string());
    let proxy3_node = net_graph.add_node(proxy3);

    let mut proxy4 = Machine::new("Proxy 4".to_string(), Some(&gray_space));
    proxy4.add_address("192.0.27.9".to_string());
    let proxy4_node = net_graph.add_node(proxy4);

    let mut target1 = Machine::new("Target 1".to_string(), Some(&red_space));
    target1.add_address("192.0.25.9".to_string());
    target1.add_address("10.7.0.2".to_string());
    let target1_node = net_graph.add_node(target1);

    let mut target2 = Machine::new("Target 2".to_string(), Some(&red_space));
    target2.add_address("10.7.0.3".to_string());
    let target2_node = net_graph.add_node(target2);
    
    let mut target3 = Machine::new("Target 3".to_string(), Some(&red_space));
    target3.add_address("10.7.0.5".to_string());
    let target3_node = net_graph.add_node(target3);

    let mut target4 = Machine::new("Target 4".to_string(), Some(&red_space));
    target4.add_address("10.7.0.6".to_string());
    let target4_node = net_graph.add_node(target4);

    net_graph.add_edge(workstation1_node, switch_node);
    net_graph.add_edge(workstation2_node, switch_node);
    net_graph.add_edge(workstation3_node, switch_node);
    net_graph.add_edge(switch_node, router_node);

    net_graph.add_edge(router_node, proxy1_node);
    net_graph.add_edge(router_node, proxy2_node);

    net_graph.add_edge(proxy1_node, proxy3_node);
    net_graph.add_edge(proxy3_node, proxy4_node);
    net_graph.add_edge(proxy2_node, proxy4_node);

    net_graph.add_edge(proxy3_node, target1_node);
    net_graph.add_edge(proxy2_node, target1_node);

    net_graph.add_edge(target1_node, target2_node);
    net_graph.add_edge(target1_node, target3_node);
    net_graph.add_edge(target2_node, target4_node);
    

    net_graph.print_graph(&mut multi_write);
}