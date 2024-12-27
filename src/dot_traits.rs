use std::rc::Rc;

pub trait DotNode {
    fn print_node(&self) -> String;
    fn name(&self) -> String;
    fn get_cluster(&self) -> &Option<Rc<Cluster>>;
}


#[derive(PartialEq, Eq, Hash)]

pub struct Cluster {
    name: String,
    color: String,
}

impl Cluster {
    pub fn new(name: String, color: String) -> Cluster {
        Cluster {
            name: name,
            color: color,

        }
    }
}