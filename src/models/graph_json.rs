extern crate serde_json;
use std::collections::HashMap;

/// JSONを出力するときにだけ使う入れ物。
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphJson {
    pub entrance: Vec<String>,
    pub nodes: Vec<NodeJson>,
}
impl GraphJson {
    pub fn new()->GraphJson{
        GraphJson{
            entrance: Vec::new(),
            nodes: Vec::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeJson {
    pub label: String,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub exit: HashMap<String,Vec<String>>,
}
impl NodeJson {
    pub fn new() -> NodeJson {
        NodeJson {
            label: "".to_string(),
            exit: HashMap::new(),
        }
    }
}
