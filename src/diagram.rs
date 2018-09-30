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
    exit_map: HashMap<String, String>,
}
impl Node {
    /// 確認用。
    pub fn get_exit_map(&self) -> &HashMap<String, String> {
        &self.exit_map
    }
    pub fn get_exit(&self, exit_label: &str) -> String {
        if self.contains_exit(&exit_label.to_string()) {
            match self.exit_map.get(exit_label) {
                Some(n) => n.to_string(),
                None => "".to_string(),
            }
        } else {
            panic!(
                "\"{}\" exit is not found. Please use contains_exit().",
                exit_label
            );
        }
    }
    pub fn contains_exit(&self, exit_label: &str) -> bool {
        self.exit_map.contains_key(exit_label)
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
    entry_point: String,
    node_map: HashMap<String, Node>,
}
impl Diagram {
    pub fn new() -> Diagram {
        Diagram {
            entry_point: "".to_string(),
            node_map: HashMap::new(),
        }
    }
    /// 確認用。
    pub fn get_node_map(&self) -> &HashMap<String, Node> {
        &self.node_map
    }
    /// クリアー。（登録したコントローラーを除く）
    pub fn clear_graph(&mut self) {
        self.entry_point = "".to_string();
        self.node_map.clear();
    }
    /// 開始ノード ラベル。
    pub fn get_entry_point(&self) -> String {
        println!("開始ノード ラベル。");
        self.entry_point.to_string()
    }
    /// 開始ノード ラベル。
    pub fn set_entry_point(&mut self, value: String) {
        self.entry_point = value;
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
    /// * `exit_map2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        label: String,
        exit_map2: HashMap<String, String>,
    ) {
        self.node_map.insert(
            label,
            Node {
                exit_map: exit_map2,
            },
        );
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
        self.entry_point = v["entry_point"].as_str().unwrap().to_string();

        for node in v["nodes"].as_array().unwrap().iter() {
            let mut exit_map: HashMap<String, String> = HashMap::new();

            if !&node["exit"].is_null() {
                for (name1, value1) in node["exit"].as_object().unwrap().iter() {
                    println!("exit: {} {}", name1.to_string(), value1.as_str().unwrap());
                    exit_map.insert(name1.to_string(), value1.as_str().unwrap().to_string());
                }
            }

            println!("insert node: [{}]", node["label"].as_str().unwrap().to_string());
            self.insert_node(
                node["label"].as_str().unwrap().to_string(),
                exit_map,
            );
            println!("neutral node contains?: {}", self.contains_node("neutral"));
            
        }
    }
}
