/// クライアント１つにつき、１つのシェルを与えます。
/// 行単位です。
///
/// コマンド例
///
/// ```
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo clippy
/// ```
use diagram::*;

/// ダイアグラム再生機。
///
/// # Members.
///
/// * `current_label` - 現在のノードのラベル。
pub struct DiagramPlayer {
    current_label: String,
}
impl Default for DiagramPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl DiagramPlayer {
    pub fn new() -> DiagramPlayer {
        DiagramPlayer {
            current_label: "".to_string(),
        }
    }

    /// 現在ノードのラベル。
    pub fn get_current(&self) -> String {
        self.current_label.to_string()
    }

    /// 現在地が遷移図の外か。
    pub fn is_out(&self) -> bool {
        self.current_label == ""
    }

    /// 遷移するぜ☆（＾～＾）
    ///
    /// # Arguments.
    ///
    /// * 'diagram' - ダイアグラムは毎回指定しろだぜ☆（＾～＾）
    /// * 'exit_label' - exitのラベル☆（＾～＾）
    pub fn forward(&mut self, diagram: &Diagram, exit_label: &str) {

        // 現在地が遷移図の外なら、入り口から入れだぜ☆（＾～＾）
        if self.is_out() {
            self.current_label = diagram.get_entry_point().to_string();
        }

        // まず ノードを取得。
        let current_node = diagram.get_node(&self.current_label);

        // 次のノード名に変更する。
        self.current_label = match current_node.get_exit_map().get(exit_label) {
            Some(n) => n.to_string(),
            None => "".to_string(),
        }
    }
}
