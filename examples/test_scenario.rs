use kifuwarabe_shell::graph::ResponseOption;
use kifuwarabe_shell::graph::*;

const GRAPH_JSON_FILE: &str = "graph.json";

// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    pub fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}

pub fn setup_graph(graph: &mut Graph<ShellVar>) {
    // コントローラーを登録。
    graph.insert_fn("do_neutral", do_neutral);
    graph.insert_fn("do_walk", do_walk);
    graph.insert_fn("do_run", do_run);

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(&GRAPH_JSON_FILE);
}

pub fn do_neutral(shell_var: &mut ShellVar, _req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Neutral.");
    res.forward("walk");
}

pub fn do_walk(shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Walk.");
    res.forward("stop");
}

pub fn do_run(shell_var: &mut ShellVar, _req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Run.");
    res.forward("stop");
}
