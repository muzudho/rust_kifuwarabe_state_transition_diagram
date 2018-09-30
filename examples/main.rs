extern crate kifuwarabe_state_transition_diagram;
/// 1行実行するだけ。
/// ```
/// ### [Windows]+[R]キー, "cmd"+[Enter]。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_state_transition_diagram
/// 
/// ### 以下のコマンドで実行。
/// cargo run --example main
/// ```
// 参考:
// https://github.com/serde-rs/json |serde_json
extern crate serde_json;
use kifuwarabe_state_transition_diagram::diagram_player::*;
use kifuwarabe_state_transition_diagram::diagram::*;

const DIAGRAM_JSON_FILE: &str = "diagram.json";

/// # テスト方法。
///
/// graph.json ファイルに書かれているスクリプトをテストします。
///
/// - 「ab cde」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Ab-NewLine.
/// - 「end xyz」と打鍵して [Enter]キーを押す。
///     End.
/// - 「xyz」と打鍵して [Enter]キーを押す。
///     Word(xyz).
/// - 「ab cde xyz」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Word(xyz).
///     Ab-NewLine.
/// - 「quit」と打鍵して [Enter]キーを押す。
///     Quit.
/// - 強制終了したいなら、[Ctrl]+[C]キー を押す。
///
/// - また、「reload」と打鍵して [Enter]キーを押す。
///     Reload.
///     graph.json ファイルを再読み込みするはず。
fn main() {

    // ダイアグラムの作成。
    let mut diagram : Diagram = Diagram::new();
    // ファイルからグラフのノード構成を読取。
    diagram.read_file(&DIAGRAM_JSON_FILE);
    println!("neutral node contains?: {}", diagram.contains_node("neutral"));

    // 内容確認出力。
    {
        println!("entry_point: {}", diagram.get_entry_point());

        println!("nodes");
        for (node_label, node) in diagram.get_node_map().iter() {
            println!("  - {}", node_label);
            for (exit_label, exit_value) in node.get_exit_map().iter() {
                println!("    - {}", exit_label);
                println!("      - {}", exit_value);
            }
        }
    }

    // ****************************************************************************************************
    //  実行。
    // ****************************************************************************************************
    // ダイアグラム再生機 の作成。
    let mut diagramPlayer = DiagramPlayer::new();
    diagramPlayer.set_diagram(&diagram);
    println!("neutral node contains?: {}", diagramPlayer.diagram.contains_node("neutral"));

    println!("Please enter command.");
    diagramPlayer.forward("walk");
    println!("Finished. shell_var.count: {}.", diagramPlayer.get_current());
}
