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
/// * `diagram` - ダイアグラム。
/// * `current_label` - 現在のノードのラベル。
pub struct DiagramPlayer {
    diagram: Diagram,
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
            diagram: Diagram::new(),
            current_label: "".to_string(),
        }
    }
    /// コマンドを1行も入力していなければ真を返します。
    pub fn get_current(&self) -> String {
        self.current_label.to_string()
    }
    pub fn is_finished (&self) -> bool {
        self.current_label == ""
    }

    /// 1行 処理するだけでいいとき。
    /// 
    /// - Quits は無効になる。
    /// 
    /// # Arguments.
    /// 
    /// * 'graph' - 
    /// * 't' - 
    /// * 'line' - 
    pub fn forward(&mut self, exit_label: &str) {

        if self.is_finished() && !self.diagram.get_entrance_vec().contains(&exit_label.to_string()) {
            // 入り口から 入ること。
            panic!("Not found {}.", exit_label);
        }

        // まず ノードを取得。
        let current_node = self.diagram.get_node(&self.current_label);

        // 次のノード名に変更する。
        self.current_label = match current_node.get_exit_map().get(exit_label) {
            Some(n) => n.to_string(),
            None => "".to_string(),
        }
    }
}
