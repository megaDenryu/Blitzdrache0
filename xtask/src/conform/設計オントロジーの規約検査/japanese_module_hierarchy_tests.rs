//! 日本語のディレクトリを含むモジュールの階層で、型の同一性の判定と設計解釈マーカーの検査が従来どおり働くことの試験。
//! クレート `a` の `src/接触.rs` と `src/接触/シーン.rs` を組み、導かれるモジュールパスが `a::接触` と `a::接触::シーン` になることも確かめる。

use std::path::Path;

use super::module_path::モジュールパス;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

fn ファイルのモジュールパスの表記(パス: &str) -> String {
    モジュールパス::ファイルのパスから求める(Path::new(パス)).表記().to_string()
}

#[test]
fn モジュールパス_日本語のディレクトリを含む階層のモジュールパスを導く() {
    assert_eq!(ファイルのモジュールパスの表記("crates/a/src/接触.rs"), "a::接触");
    assert_eq!(ファイルのモジュールパスの表記("crates/a/src/接触/シーン.rs"), "a::接触::シーン");
    assert_eq!(ファイルのモジュールパスの表記("crates/a/src/接触/mod.rs"), "a::接触");
}

#[test]
fn 構文解析_日本語のディレクトリの下の型にもコマンドの規約を当てる() {
    let 上 = ソース("crates/a/src/接触.rs", "#[path = \"接触/シーン.rs\"]\nmod シーン;\n");
    let 下 = ソース("crates/a/src/接触/シーン.rs", "pub struct 衝突の指示;\nimpl Mコマンド for 衝突の指示 {}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![上, 下]);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("Mコマンド `衝突の指示` は enum 定義が必要です"));
}

#[test]
fn 構文解析_日本語のディレクトリの下の列挙型のコマンドは違反にならない() {
    let 上 = ソース("crates/a/src/接触.rs", "#[path = \"接触/シーン.rs\"]\nmod シーン;\n");
    let 下 = ソース("crates/a/src/接触/シーン.rs", "pub enum 衝突の指示 {\n    止める,\n}\nimpl M不変データ for 衝突の指示 {}\nimpl Mコマンド for 衝突の指示 {}\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![上, 下]).is_empty());
}

#[test]
fn 構文解析_日本語のディレクトリの下と上に同名の型があれば実装と同じファイルの定義を採る() {
    let 上 = ソース("crates/a/src/接触.rs", "#[path = \"接触/シーン.rs\"]\nmod シーン;\npub struct 位置<'a> {\n    値: &'a f32,\n}\n");
    let 下 = ソース("crates/a/src/接触/シーン.rs", "pub struct 位置 {\n    pub 東: f32,\n}\nimpl M不変データ for 位置 {}\nimpl M状態 for 位置 {}\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![上, 下]).is_empty());
}
