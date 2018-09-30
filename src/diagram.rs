/// アプリケーション１つにつき、１つのグラフを持ちます。
// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `fn_name` - 次の exit を返すコールバック関数名。
/// * `exit_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node {
    exits: HashMap<String, String>,
}
impl Node {
    /// 確認用。
    pub fn get_exits_map(&self) -> &HashMap<String, String> {
        &self.exits
    }
    pub fn get_exits(&self, exit_label: &str) -> String {
        if self.contains_exits(&exit_label.to_string()) {
            match self.exits.get(exit_label) {
                Some(n) => n.to_string(),
                None => "".to_string(),
            }
        } else {
            panic!(
                "\"{}\" exit is not found. Please use contains_exits().",
                exit_label
            );
        }
    }
    pub fn contains_exits(&self, exit_label: &str) -> bool {
        self.exits.contains_key(exit_label)
    }
}

/// 状態遷移図。
/// 
/// # Parameters.
///
/// * `entrance_vec` - スタート地点となる ノード ラベル。
/// * `node_map` - ノードのマップ。ラベルがキー。
#[derive(Default)]
pub struct Diagram {
    entrance_vec: Vec<String>,
    node_map: HashMap<String, Node>,
}
impl Diagram {
    pub fn new() -> Diagram {
        Diagram {
            entrance_vec: Vec::new(),
            node_map: HashMap::new(),
        }
    }
    /// 確認用。
    pub fn get_node_map(&self) -> &HashMap<String, Node> {
        &self.node_map
    }
    /// クリアー。（登録したコントローラーを除く）
    pub fn clear_graph(&mut self) {
        self.node_map.clear();
        self.entrance_vec.clear();
    }
    pub fn get_entrance_vec(&self) -> &Vec<String> {
        &self.entrance_vec
    }
    pub fn set_entrance_vec(&mut self, entrance_vec2: Vec<String>) {
        self.entrance_vec = entrance_vec2;
    }
    pub fn get_node(&self, label: &str) -> &Node {
        if self.contains_node(&label.to_string()) {
            &self.node_map[label]
        } else {
            panic!("\"{}\" node is not found.", label);
        }
    }
    pub fn contains_node(&self, label: &str) -> bool {
        self.node_map.contains_key(&label.to_string())
    }
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    /// * `node` - ノードです。
    /// * `exits2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        label: String,
        exits2: HashMap<String, String>,
    ) {
        self.node_map.insert(
            label,
            Node {
                exits: exits2,
            },
        );
    }

    /// JSONオブジェクトを、文字列のハッシュマップに変換。
    ///
    /// # Arguments.
    ///
    /// * 'v' - Json object.
    /// * 'str_vec' - let str_vec = Vec::new();
    fn object_to_map(&self, obj: &Value, map0: &mut HashMap<String, String>) {
        if !obj.is_null() {
            for (name1, value1) in obj.as_object().unwrap().iter() {
                map0.insert(name1.to_string(), value1.to_string());
            }
        }
    }
    /// ファイル読み込み
    pub fn read_file(&mut self, file: &str) {
        self.clear_graph();

        let mut file = match File::open(file) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let v: Value = match serde_json::from_str(&data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        // 文字列に変換する。
        {
            self.entrance_vec.clear();
            array_to_str_vec(&v["entrance"], &mut self.entrance_vec);
        }

        for node in v["nodes"].as_array().unwrap().iter() {
            let mut exit_map: HashMap<String, String> = HashMap::new();
            self.object_to_map(&node["exit"], &mut exit_map);
            self.insert_node(
                node["label"].as_str().unwrap().to_string(),
                exit_map,
            );
        }
    }
}

/// JSON配列を、文字列の配列に変換。
///
/// # Arguments.
///
/// * 'v' - Json array.
/// * 'str_vec' - let str_vec = Vec::new();
fn array_to_str_vec(v: &Value, str_vec: &mut Vec<String>) {
    let value_vec: Vec<Value> = v.as_array().unwrap().to_vec();
    for node_label in value_vec {
        str_vec.push(node_label.as_str().unwrap().to_string());
    }
}
