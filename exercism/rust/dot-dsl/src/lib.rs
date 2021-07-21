// modified desdulianto's solution
use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs.iter() {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
    };
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    impl_attrs!();

    pub fn get_node(&self, node: &str) -> Option<&Node> {
        self.nodes.iter().find(|x| x.name == node)
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    impl_attrs!();

    pub fn get_attr(&self, attr: &str) -> Option<&str> {
        self.attrs.get(attr).map(|x| x.as_str())
    }
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            attrs: HashMap::new(),
        }
    }

    impl_attrs!();
}

// tests expect attrs to be Strings, so those aren't &str
// test_graph_stores_attributes fails here. Temporary freed error
// #[derive(Default)]
// pub struct Graph<'a> {
//     pub nodes: Vec<Node<'a>>,
//     pub edges: Vec<Edge<'a>>,
//     pub attrs: HashMap<String, String>,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Node<'a> {
//     pub name: &'a str,
//     pub attrs: HashMap<String, String>,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Edge<'a> {
//     pub from: &'a str,
//     pub to: &'a str,
//     pub attrs: HashMap<String, String>,
// }

// macro_rules! impl_attrs {
//     () => {
//         pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
//             for (k, v) in attrs.iter() {
//                 self.attrs.insert(k.to_string(), v.to_string());
//             }
//             self
//         }
//     };
// }

// impl<'a> Graph<'a> {
//     pub fn new() -> Self {
//         Graph {
//             nodes: Vec::new(),
//             edges: Vec::new(),
//             attrs: HashMap::new(),
//         }
//     }

//     pub fn with_nodes(mut self, nodes: &'a [Node]) -> Self {
//         self.nodes = nodes.to_vec();
//         self
//     }

//     pub fn with_edges(mut self, edges: &'a [Edge]) -> Self {
//         self.edges = edges.to_vec();
//         self
//     }

//     impl_attrs!();

//     pub fn get_node(&self, node: &str) -> Option<&Node> {
//         self.nodes.iter().find(|x| x.name == node)
//     }
// }

// impl<'a> Node<'a> {
//     pub fn new(name: &'a str) -> Self {
//         Node {
//             name,
//             attrs: HashMap::new(),
//         }
//     }

//     impl_attrs!();

//     pub fn get_attr(&self, attr: &str) -> Option<&str> {
//         self.attrs.get(attr).map(|x| x.as_str())
//     }
// }

// impl<'a> Edge<'a> {
//     pub fn new(from: &'a str, to: &'a str) -> Self {
//         Self {
//             from,
//             to,
//             attrs: HashMap::new(),
//         }
//     }

//     impl_attrs!();
// }

pub mod graph {
    pub use crate::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use crate::Edge;
        }
        pub mod node {
            pub use crate::Node;
        }
    }
}
