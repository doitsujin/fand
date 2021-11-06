use std::result;
use std::str::Chars;

pub type Result<T> = result::Result<T, String>;
pub type CharPtr<'a> = (Option<char>, Chars<'a>);

pub struct Parser;

pub type Nodes = Vec<Node>;
pub enum Node {
    Node(String, Nodes),
    Text(String),
}

impl Node {
    // Creates a string from a node
    pub fn dump(&self) -> String {
        self.do_dump(&String::new())
    }

    // Creates a string from a sequence of nodes
    pub fn dump_nodes(nodes: &Nodes) -> String {
        Node::do_dump_nodes(nodes, &String::new())
    }

    // Dumps a single node with given indentation
    fn do_dump(&self, indent: &String) -> String {
        let new_indent = format!("  {}", indent);

        match *self {
            Node::Node(ref name, ref content) => format!(
                "{}({}{})",
                indent,
                name,
                Node::do_dump_nodes(content, &new_indent)
            ),
            Node::Text(ref text) => format!("{}\"{}\"", indent, text),
        }
    }

    // Dumps a sequence of nodes with given indentation
    fn do_dump_nodes(content: &Nodes, indent: &String) -> String {
        content.iter().fold(String::new(), |mut s, n| {
            s.push('\n');
            s.push_str(&n.do_dump(indent));
            s
        })
    }
}

impl Parser {
    // Parses a full document
    pub fn parse_document(doc: &str) -> Result<Nodes> {
        let mut s = doc.chars();
        let c = s.next();
        Parser::parse_nodes((c, s)).map(|(n, _)| n)
    }

    // Parses a sequence of nodes
    fn parse_nodes<'a>(p: CharPtr<'a>) -> Result<(Nodes, CharPtr<'a>)> {
        let (mut c, mut s) = Parser::skip_whitespace(p);
        let mut result = Nodes::new();

        loop {
            match c {
                Some('"') | Some('(') => {
                    // Parse the given node
                    let (node, p) = Parser::parse_content((c, s))?;
                    result.push(node);

                    // Whitespace may occur after the end of our content
                    let (ci, si) = Parser::skip_whitespace(p);
                    c = ci;
                    s = si;
                }
                _ => return Ok((result, (c, s))),
            }
        }
    }

    // Parses node content
    fn parse_content<'a>(p: CharPtr<'a>) -> Result<(Node, CharPtr<'a>)> {
        let (c, mut s) = p;
        match c {
            Some('"') => Parser::parse_text((s.next(), s)),
            Some('(') => Parser::parse_node((s.next(), s)),
            _ => Err("Expected \" or (".to_string()),
        }
    }

    // Parses a single node, including its child nodes
    fn parse_node<'a>(p: CharPtr<'a>) -> Result<(Node, CharPtr<'a>)> {
        let (name, p1) = Parser::parse_identifier(p)?;
        let (nodes, p2) = Parser::parse_nodes(p1)?;
        let (c, mut s) = p2;
        match c {
            Some(')') => Ok((Node::Node(name, nodes), (s.next(), s))),
            _ => Err("Expected ')'".to_string()),
        }
    }

    // Reads an identifier. Identifiers can consist
    // of anything but ()" and whitespace.
    fn parse_identifier<'a>(p: CharPtr<'a>) -> Result<(String, CharPtr<'a>)> {
        let mut result = String::new();
        let (mut c, mut s) = p;

        loop {
            match c {
                None | Some('"') | Some('(') => {
                    return Err("Expected ')' or whitespace".to_string())
                }
                Some(')') | Some(' ') | Some('\x09') | Some('\x0A') | Some('\x0D') => {
                    return Ok((result, (c, s)))
                }
                Some(c) => result.push(c),
            }
            c = s.next();
        }
    }

    // Parses a simple text node
    fn parse_text<'a>(p: CharPtr<'a>) -> Result<(Node, CharPtr<'a>)> {
        let (mut c, mut s) = p;
        let mut result = String::new();
        loop {
            match c {
                None => return Err("Expected \"".to_string()),
                Some('"') => return Ok((Node::Text(result), (s.next(), s))),
                Some('\\') => result.push(s.next().ok_or("Expected character".to_string())?),
                Some(c) => result.push(c),
            }
            c = s.next();
        }
    }

    // Skips all whitespace characters
    fn skip_whitespace<'a>(p: CharPtr<'a>) -> CharPtr<'a> {
        let (mut c, mut s) = p;
        loop {
            match c {
                Some(' ') | Some('\x09') | Some('\x0D') | Some('\x0A') => c = s.next(),
                _ => return (c, s),
            }
        }
    }
}
