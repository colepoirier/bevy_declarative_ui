use std::collections::HashMap;

use crate::model::Property;

// pub trait Node {
//     fn create_element(html_tag: String);
//     fn create_element_ns(namespace: String, html_tag: String);
//     fn add_event_listner(event_kind: String, call_back: dyn FnOnce());
//     fn set_attr(key: String, val: String);
//     fn remove_attr(key: String);
//     fn set_attr_ns(namespace: String, key: String, val: String);
//     fn remove_attr_ns(namespace: String, key: String);
// }

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Fact {
    Attr(FactContent),
    AttrNS(FactContent),
    Event(FactContent),
    Prop(FactContent),
    Style(FactContent),
}

impl Default for Fact {
    fn default() -> Self {
        Fact::Attr(FactContent::default())
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct FactContent {
    key: String,
    val: String,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum VNodeKind {
    Text,
    Node,
    KeyedNode,
    Thunk,
    Tagger,
    Custom,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PatchKind {
    Redraw,
    Facts,
    Text,
    Thunk,
    Tagger,
    RemoveLast,
    Append,
    Reorder,
    Remove,
    Custom,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum EntryKind {
    Insert,
    Remove,
    Move,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct VdomObj {
    kind: VNodeKind,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct VTextNode {
    text: String,
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct VNode {
    pub tag: String,
    pub facts: Vec<Fact>,
    pub kids: Vec<VNode>,
    pub kid_count: u32,
    pub namespace: String,
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct Model; // probably the elm model object

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct Render; // probably a function

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct CustomVNode {
    pub facts: Vec<Fact>,
    pub model: Model,
    pub render: Render,
    pub diff: i32,
}

#[derive(Debug, Default, Clone)]
pub struct OrganizedFacts {
    props: HashMap<String, String>,
    attrs: HashMap<String, String>,
}

pub fn organize_facts(fact_list: Vec<Fact>) -> OrganizedFacts {
    let mut facts = OrganizedFacts::default();
    for fact in fact_list {
        match fact {
            Fact::Prop(FactContent { key, val }) => {
                if key == "className" {
                    add_class(&mut facts.props, key, val);
                } else {
                    facts.props.insert(key, val);
                }
            }
            Fact::Attr(FactContent { key, val }) => {
                if key == "class" {
                    add_class(&mut facts.attrs, key, val)
                } else {
                    facts.attrs.insert(key, val);
                }
            }
            _ => continue,
        }
    }
    return facts;
}

pub fn add_class(
    facts: &mut HashMap<String, String>,
    key: String,
    val: String,
) {
    if let Some(v) = facts.get_mut(&key) {
        *v = format!("{} {}", v, val);
    } else {
        facts.insert(key, val);
    }
}

// pub fn apply_facts(
//     node: impl Node,
//     event_node: impl Node,
//     facts: OrganizedFacts,
// ) {
// }

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Node {
    pub tag: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<NodeType>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NodeType {
    Node(Node),
    KeyedNode(String, Node),
    Text(String),
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Node(Node {
            tag: "div".to_string(),
            attrs: vec![],
            children: vec![],
        })
    }
}

pub fn text(txt: String) -> NodeType {
    NodeType::Text(txt)
}

pub fn node(
    tag: String,
    attrs: Vec<Attribute>,
    children: Vec<NodeType>,
) -> Node {
    Node {
        tag,
        attrs,
        children,
    }
}

pub fn keyed_node(
    key: String,
    tag: String,
    attrs: Vec<Attribute>,
    children: Vec<NodeType>,
) -> NodeType {
    NodeType::KeyedNode(
        key,
        Node {
            tag,
            attrs,
            children,
        },
    )
}

pub fn property(property: Property) -> Attribute {
    Attribute(format!("{}={}", property.0, property.1))
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
pub struct Attribute(pub String);

pub mod html {
    use crate::vdom;
    use vdom::{node, Node, NodeType};

    // pub type Node = Node;

    pub fn text(txt: String) -> NodeType {
        vdom::text(txt)
    }

    pub fn div(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "div".to_string(),
            attrs,
            children,
        }
    }

    // paragraph html tag
    pub fn p(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "p".to_string(),
            attrs,
            children,
        }
    }

    // strikethrough html tag
    pub fn s(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "s".to_string(),
            attrs,
            children,
        }
    }

    // underline html tag
    pub fn u(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "u".to_string(),
            attrs,
            children,
        }
    }

    pub mod attributes {
        use crate::vdom;

        pub fn class(cls: String) -> vdom::Attribute {
            vdom::Attribute(cls)
        }

        pub fn style(k: String, v: String) -> vdom::Attribute {
            vdom::Attribute(format!("{}={}", k, v))
        }

        pub fn src(s: String) -> vdom::Attribute {
            style("src".to_string(), s)
        }

        pub fn alt(description: String) -> vdom::Attribute {
            style("alt".to_string(), description)
        }

        pub fn href(url: String) -> vdom::Attribute {
            style("href".to_string(), url)
        }

        pub fn rel(r: String) -> vdom::Attribute {
            style("rel".to_string(), r)
        }

        pub fn target(t: String) -> vdom::Attribute {
            style("target".to_string(), t)
        }

        pub fn download(file_name: String) -> vdom::Attribute {
            style("download".to_string(), file_name)
        }
    }
}
