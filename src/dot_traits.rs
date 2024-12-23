pub trait DotNode {
    fn print_node(&self) -> String;
    fn name(&self) -> String;
}

pub trait DotCluster {
    fn print_cluster(&self) -> String;
}

pub trait DotEdge {
    fn print_edge(&self) -> String;
}
