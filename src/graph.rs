/// アプリケーション１つにつき、１つのグラフを持ちます。
// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::collections::HashMap;
use std::fs::OpenOptions;

use models::graph_json::*;

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `exit_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node {
    // 特殊な任意の名前 '#newline'
    exits: HashMap<String, Vec<String>>,
}
impl Node {
    /// 確認用。
    pub fn get_exits_map(&self) -> &HashMap<String, Vec<String>> {
        &self.exits
    }
    pub fn get_exits(&self, name: &str) -> &Vec<String> {
        if self.contains_exits(&name.to_string()) {
            &self.exits[name]
        } else {
            panic!(
                "\"{}\" exit is not found. Please use contains_exits().",
                name
            );
        }
    }
    pub fn contains_exits(&self, name: &str) -> bool {
        self.exits.contains_key(name)
    }
}

pub fn empty_controller<T>(_t: &mut T) {}

/// # Parameters.
///
/// * `node_map` - 複数件のトークンです。
/// * `entrance_vec` - カンマ区切りの登録ノード名です。
#[derive(Default)]
pub struct Graph {
    entrance_vec: Vec<String>,
    /// 特殊なノード名
    /// '#else' 一致するトークンが無かったときに呼び出されるコールバック関数です。
    node_map: HashMap<String, Node>,
}
impl Graph {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Graph {
        Graph {
            node_map: HashMap::new(),
            entrance_vec: Vec::new(),
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
        exits2: HashMap<String, Vec<String>>,
    ) {
        self.node_map.insert(
            label,
            Node {
                exits: exits2,
            },
        );
    }

    /// JSON配列を、文字列の配列に変換。
    ///
    /// # Arguments.
    ///
    /// * 'v' - Json array.
    /// * 'str_vec' - let str_vec = Vec::new();
    fn array_to_str_vec(&self, v: &Value, str_vec: &mut Vec<String>) {
        let value_vec: Vec<Value> = v.as_array().unwrap().to_vec();
        for node_label in value_vec {
            str_vec.push(node_label.as_str().unwrap().to_string());
        }
    }
    /// JSONオブジェクトを、文字列のハッシュマップに変換。
    ///
    /// # Arguments.
    ///
    /// * 'v' - Json object.
    /// * 'str_vec' - let str_vec = Vec::new();
    fn object_to_map(&self, obj: &Value, map0: &mut HashMap<String, Vec<String>>) {
        if !obj.is_null() {
            for (name1, array1) in obj.as_object().unwrap().iter() {
                let mut array2: Vec<String> = Vec::new();
                for item1 in array1.as_array().unwrap().iter() {
                    array2.push(item1.as_str().unwrap().to_string());
                }
                map0.insert(name1.to_string(), array2);
            }
        }
    }
    /// ファイル読み込み
    pub fn read_graph_file(&mut self, file: &str) {
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
        let mut entrance_vec: Vec<String> = Vec::new();
        self.array_to_str_vec(&v["entrance"], &mut entrance_vec);
        self.set_entrance_vec(entrance_vec);

        for node in v["nodes"].as_array().unwrap().iter() {
            let mut entrance_map: HashMap<String, Vec<String>> = HashMap::new();
            self.object_to_map(&node["exit"], &mut entrance_map);
            self.insert_node(
                node["label"].as_str().unwrap().to_string(),
                entrance_map,
            );
        }
    }
    /// ファイル上書き書込。
    /// https://qiita.com/garkimasera/items/0442ee896403c6b78fb2 |JSON文字列と構造体の相互変換
    pub fn save_graph_file(&mut self, file: &str) {
        // 移し替え。
        let mut graph_json = GraphJson::new();
        // エントランス
        for node_label in &self.entrance_vec {
            graph_json.entrance.push(node_label.to_string());
        }
        // ノード
        for (node_label, node) in &self.node_map {
            let mut node_json = NodeJson::new();            
            node_json.label = node_label.to_string();

            for (exits_label,node_vec) in node.get_exits_map().iter() {
                let mut vec = Vec::new();
                for exits_node in node_vec.iter() {
                    vec.push(exits_node.to_string());
                }
                node_json.exit.insert(exits_label.to_string(), vec);
            }

            graph_json.nodes.push(node_json);
        }
        let json_str = serde_json::to_string(&graph_json).unwrap();

        // 上書き書込。
        match &mut OpenOptions::new().create(true).write(true).truncate(true).open(file) {
            Ok(contents_file) => contents_file.write_all(json_str.as_bytes()),
            Err(err) => panic!("Log file open (write mode) error. {}", err),
        };
    }
}
