//! ts-rsで、editor_serverの素データ型からTypeScriptの型契約の本文を組み立てる。ここでは文字列を作るだけで、
//! ファイルへは触れない。組み立てた結果を、書き出す側(`bin/contract_export.rs`)と検査する側(鮮度検査テスト)の
//! 両方が呼ぶ。
//!
//! 生成するファイルは2つあり、1つのモジュールが1つの生成ファイルの本文を所有する。`生存確認契約.ts`はこの
//! モジュールが、`編集資源契約.ts`は`editor_resource`が持つ。ここに残すのは、2つが共有する手書き禁止の注記と
//! 宣言の並べ方だけである。
//!
//! `TS::decl()`は単体のimport文を作らないため、互いに参照し合う型は同じ1本のファイルへまとめる
//! (手本: GameScriptingTheoryの同名モジュール)。生存確認応答だけは何にも依存しないため、別ファイルのまま残す。

mod editor_resource;

use ts_rs::{Config, TS};

pub use editor_resource::編集資源契約の本文を組み立てる;

const 手書き禁止の注記: &str = "// このファイルはts-rsによる生成物である。手で編集しない。\n\
// 生成し直す手順（リポジトリルートで実行する）:\n\
//   cargo xtask contract-export\n";

pub fn 契約ファイルの本文を組み立てる() -> String {
    let 設定 = Config::new();
    let 型宣言一覧: [String; 1] = [<crate::生存確認応答 as TS>::decl(&設定)];
    本文を組み立てる(&型宣言一覧)
}

fn 本文を組み立てる(型宣言一覧: &[String]) -> String {
    let mut 本文 = String::from(手書き禁止の注記);
    本文.push('\n');
    for 宣言 in 型宣言一覧 {
        本文.push_str("export ");
        本文.push_str(宣言);
        本文.push('\n');
    }
    本文
}
