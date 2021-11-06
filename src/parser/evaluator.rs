use crate::parser::Node;
use std::collections::HashMap;
use std::rc::Rc;

// Evaluator
//
// Processes either one single node or a sequence
// of nodes and returns an object of a given type.
pub trait Evaluator<T> {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<T, String>;
}

// Tag-based evaluator
//
// Fowards nodes to a given parser, depending on
// the node name. Text nodes will cause errors.
pub struct TagEvaluator<T> {
    parsers: HashMap<String, Rc<dyn Evaluator<T>>>,
}

impl<T> TagEvaluator<T> {
    pub fn new() -> TagEvaluator<T> {
        TagEvaluator {
            parsers: HashMap::new(),
        }
    }

    // Adds named evaluator
    pub fn add(&mut self, s: &str, p: Rc<dyn Evaluator<T>>) {
        self.parsers.insert(s.to_string(), p);
    }

    // Forwards a single node to the given evaluator
    pub fn parse_node(&self, node: &Node) -> Result<T, String> {
        match *node {
            Node::Node(ref title, ref content) => self
                .parsers
                .get(title)
                .ok_or(format!("Unknown node type: {}", title))?
                .parse_nodes(content),
            _ => Err("Expected node".to_string()),
        }
    }
}

impl<T> Evaluator<Vec<T>> for TagEvaluator<T> {
    // Parses a sequence of nodes and returns a sequence of elements
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Vec<T>, String> {
        let mut result: Vec<T> = Vec::new();
        for n in nodes.iter() {
            result.push(self.parse_node(n)?);
        }
        Ok(result)
    }
}
