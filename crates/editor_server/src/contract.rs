//! ts-rsで、編集資源の素データ型からTypeScriptの型契約の本文を組み立てる。
//! この関数の結果を、書き出す側（bin/contract_export.rs）と検査する側
//! （鮮度検査テスト）の両方が呼ぶ。ここでは文字列を作るだけで、ファイルへは触れない。
//! 形はGameScriptingTheoryの`crates/editor_server/src/contract.rs`に揃える。

use ts_rs::{Config, TS};

const 手書き禁止の注記: &str = "// このファイルはts-rsによる生成物である。手で編集しない。\n\
// 生成し直す手順（リポジトリルートで実行する）:\n\
//   cargo xtask contract-export\n";

pub fn 契約ファイルの本文を組み立てる() -> String {
    let 設定 = Config::new();
    let 型宣言一覧: [String; 1] = [<crate::生存確認応答 as TS>::decl(&設定)];
    let mut 本文 = String::from(手書き禁止の注記);
    本文.push('\n');
    for 宣言 in 型宣言一覧 {
        本文.push_str("export ");
        本文.push_str(&宣言);
        本文.push('\n');
    }
    本文
}
