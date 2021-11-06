use crate::parser::Node;
use std::str::FromStr;

pub fn get_text_node<'a>(
    tag_name: &str,
    nodes: &'a [Node],
    id: usize,
) -> Result<&'a String, String> {
    match nodes.get(id) {
        Some(&Node::Text(ref s)) => Ok(s),
        _ => Err(format!("({}): Missing text argument", tag_name)),
    }
}

pub fn get_num_node<'a, T: FromStr>(
    tag_name: &str,
    nodes: &'a [Node],
    id: usize,
) -> Result<T, String> {
    get_text_node(tag_name, nodes, id)?
        .parse::<T>()
        .map_err(|_| "Invalid number".to_string())
}

pub fn get_node<'a>(tag_name: &str, nodes: &'a [Node], id: usize) -> Result<&'a Node, String> {
    nodes
        .get(id)
        .ok_or(format!("({}): Missing node argument", tag_name))
}
