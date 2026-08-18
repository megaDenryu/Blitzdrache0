//! コマンド選択後、使い方を表示して引数の行を1つ読む工程。空Enterのときは引数なしの実行として
//! 扱い、引用符が閉じていない行は破れを表示して読み直させる(語への分解の規則はargument_lineが持つ)。

use crate::command_ui::command_catalog::コマンド項目;

use super::argument_line;

pub(crate) fn 引数の行を読む(項目: &コマンド項目) -> Vec<String> {
    println!();
    println!("使い方: cargo xtask {}", 項目.使い方の行());
    println!("空白を含む引数は\"...\"で囲む。");
    loop {
        println!("引数を空白区切りで入力してEnter (空Enterなら引数なしで実行):");
        let mut 行 = String::new();
        if std::io::stdin().read_line(&mut 行).is_err() {
            return Vec::new();
        }
        match argument_line::語へ分ける(&行) {
            Ok(語一覧) => return 語一覧,
            Err(破れ) => println!("{破れ}"),
        }
    }
}
