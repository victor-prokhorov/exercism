pub mod graph {
    use std::collections::HashMap;
    
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                pub attrs: HashMap<String, String>,
                pub a: String,
                pub b: String,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Edge {
                    Edge {
                        attrs: HashMap::new(),
                        a: a.to_string(),
                        b: b.to_string(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self, a: &str) -> Option<&str> {
                    self.attrs.get(a).map(|s| s.as_str())
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub title: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(title: &str) -> Node {
                    Node {
                        title: title.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
            pub fn attr(&self, a: &str) -> Option<&str> {
                self.attrs.get(a).map(|s| s.as_str())
            }

            }
        }
    }
    use graph_items::node::Node;
    use graph_items::edge::Edge;
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    
    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    pub fn node(&self, title: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.title == title)
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
            self.edges = edges.to_vec();
            self
        }
    }
}
