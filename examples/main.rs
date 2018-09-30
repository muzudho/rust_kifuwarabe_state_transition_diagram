/// 1行実行するだけ。
/// ```
/// ### [Windows]+[R]キー, "cmd"+[Enter]。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_state_transition_diagram
///
/// ### 以下のコマンドで実行。
/// cargo run --example main
/// ```
extern crate kifuwarabe_state_transition_diagram;

// 参考:
// https://github.com/serde-rs/json |serde_json
extern crate serde_json;
use kifuwarabe_state_transition_diagram::diagram::*;
use kifuwarabe_state_transition_diagram::diagram_player::*;

const DIAGRAM_JSON_FILE: &str = "diagram.json";

/// # テスト方法。
///
/// 実行するだけ。
///
/// ```
/// Start!
/// walk
///  --> walking.
/// stop
///  --> neutral.
/// run
///  --> running.
/// stop
///  --> neutral.
/// Finished.
/// ```
fn main() {
    // ダイアグラムの作成。
    let mut diagram: Diagram = Diagram::new();
    // ファイルからグラフのノード構成を読取。
    diagram.read_file(&DIAGRAM_JSON_FILE);

    // 内容確認出力。
    {
        println!("entry_point: {}", diagram.get_entry_point());

        println!("nodes");
        for (node_label, node) in diagram.get_node_map().iter() {
            println!("  - {}", node_label);
            for (exit_label, exit_value) in node.get_exit_map().iter() {
                println!("    | {}", exit_label);
                println!("    +----> {}", exit_value);
            }
        }
    }

    // ****************************************************************************************************
    //  実行。
    // ****************************************************************************************************
    // ダイアグラム再生機 の作成。
    let mut diagramPlayer = DiagramPlayer::new();

    println!("Start!");

    println!("walk");
    diagramPlayer.forward(&diagram, "walk");
    println!(" --> {}.", diagramPlayer.get_current());

    println!("stop");
    diagramPlayer.forward(&diagram, "stop");
    println!(" --> {}.", diagramPlayer.get_current());

    println!("run");
    diagramPlayer.forward(&diagram, "run");
    println!(" --> {}.", diagramPlayer.get_current());

    println!("stop");
    diagramPlayer.forward(&diagram, "stop");
    println!(" --> {}.", diagramPlayer.get_current());

    println!("Finished.");
}
