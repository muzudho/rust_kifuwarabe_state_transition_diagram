use kifuwarabe_shell::graph::ResponseOption;
use kifuwarabe_shell::graph::*;

const DIAGRAM_JSON_FILE: &str = "diagram.json";

pub fn setup_graph(diagram: &mut Diagram<ShellVar>) {
    // ファイルからグラフのノード構成を読取。
    diagram.read_file(&DIAGRAM_JSON_FILE);
}
