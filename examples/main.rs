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
    // ダイアグラム再生機 の作成。
    let mut diagramPlayer = DiagramPlayer::new();

    // ダイアグラムの作成。
    let mut diagram : Diagram = Diagram::new();
    // ファイルからグラフのノード構成を読取。
    diagram.read_file(&DIAGRAM_JSON_FILE);

    // 内容確認出力。
    {
        println!("entrance");
        for node in diagram.get_entrance_vec().iter() {
            println!("  - {}", node);
        }

        println!("nodes");
        for (node_label, node) in diagram.get_node_map().iter() {
            println!("  - {} {}", node_label, node.get_token());
            for (exits_label, exits_vec) in node.get_exits_map().iter() {
                println!("    - {}", exits_label);
                for exits_item in exits_vec.iter() {
                    println!("      - {}", exits_item);
                }
            }
        }
    }

    // ****************************************************************************************************
    //  実行。
    // ****************************************************************************************************
    println!("Please enter command.");
    shell.execute_line(&mut graph, &mut shell_var, "ab cde xyz");
    println!("Finished. shell_var.count: {}.", shell_var.count);
}
