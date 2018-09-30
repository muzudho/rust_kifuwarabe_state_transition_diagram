# What is rust_kifuwarabe_state_transition_diagram?
状態遷移を プログラム言語に埋め込むの 嫌だろ。
外部ファイル に出そうぜ☆（＾～＾）  

## diagram.json

ファイル名は任意で。 JSON形式 だぜ。

```
{
    "entry_point": "neutral",
    "nodes": [
        {
            "label": "neutral",
            "exit": {
                "walk": "walking",
                "run": "running"
            }
        },
        {
            "label": "walking",
            "exit": {
                "stop": "neutral"
            }
        },
        {
            "label": "running",
            "exit": {
                "stop": "neutral"
            }
        }
    ]
}
```

フォーマットはこれだけ。

- ```entry_point``` が迷路の入り口にあたる。１つしか書けない。
- ```nodes``` が 迷路の各部屋にあたる。部屋はどこかの部屋とつながることができる。
- ```label``` が 迷路の部屋名にあたる。
- ```exit``` が 別の部屋とのつながり を書いている。walk というドアは walking 部屋につながっている感じだぜ。１つの部屋に ドアはたくさん付けれるぜ。

# Instration.

```
[dependencies]
serde_json = "1.0"

[dependencies.kifuwarabe_shell]
git = "https://github.com/muzudho/rust_kifuwarabe_state_transition_diagram.git"
rev = "bf0f994... Please get new rev from git hub."
```

rev は Git hub を見て新しいのを入れろだぜ☆（＾～＾）

# How to use 

## ファイルの冒頭。

```
extern crate kifuwarabe_state_transition_diagram;
extern crate serde_json;
use kifuwarabe_state_transition_diagram::diagram::*;
use kifuwarabe_state_transition_diagram::diagram_player::*;
```

## ダイアグラム読込。

```
const DIAGRAM_JSON_FILE: &str = "diagram.json";

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
}
```

まず ファイルが読み込めていることを確認しろだぜ。

## 実行

```
pub fn() {

    // ～ 略(Omit) ～

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
```

あとは ダイアグラム プレイヤーというのが実行してくれる。

- .forward() - 遷移だぜ。ドアの名前を指定しろだぜ。
- .get_current() - 現在、居るところの 迷路の部屋のラベルだな。

これだけ。

# そのほかの話題。

もっと複雑なやつで、パーサーも作ってるんで 興味があったら見てみろだぜ。
[rust_kifuwarabe_shell](https://github.com/muzudho/rust_kifuwarabe_shell)
