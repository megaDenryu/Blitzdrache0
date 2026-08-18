//! コマンド選択後、使い方を表示して引数の行を1つ読む工程。空Enterのときは引数なしの実行として扱う。

use crate::command_ui::command_catalog::コマンド項目;

pub(crate) fn 引数の行を読む(項目: &コマンド項目) -> Vec<String> {
    println!();
    println!("使い方: cargo xtask {}", 項目.使い方の行());
    println!("引数を空白区切りで入力してEnter (空Enterなら引数なしで実行):");
    let mut 行 = String::new();
    if std::io::stdin().read_line(&mut 行).is_err() {
        return Vec::new();
    }
    行.split_whitespace().map(str::to_string).collect()
}
