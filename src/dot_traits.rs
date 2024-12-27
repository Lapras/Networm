use std::rc::Rc;
use crate::writers::Writer;

pub trait DotNode {
    fn print_node(&self) -> String;
    fn name(&self) -> String;
    fn get_cluster(&self) -> &Option<Rc<DotCluster>>;
}


#[derive(PartialEq, Eq, Hash)]

pub struct DotCluster {
    pub name: String,
    pub color: String,
}

impl DotCluster {
    pub fn new(name: &str, color: &str) -> DotCluster {
        DotCluster {
            name: name.to_string(),
            color: color.to_string(),
        }
    }

    pub fn print_cluster_style<W: Writer>(&self, writer: &mut W, indent: i32) {
        writer.writeln(&format!("label = \"{}\"", self.name), indent);
        writer.writeln("style = solid;", indent);
        writer.writeln(&format!("color = {}", self.color), indent);
    }
}