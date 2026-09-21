//! `モジュール宣言` の抽出と、宣言が答える論理のモジュールパスと対象の物理ファイルの試験。
//! 論理と物理が一致する3つの例と、一致しない2つの例を、導かれるモジュールパスの表記で固定する。

use std::path::Path;

use super::module_declaration::{モジュール宣言, 本体の形, 置き場の指定};
use super::module_declaration_extract::モジュール宣言の抽出;
use super::module_path::モジュールパス;

fn 抽出(宣言元: &str, 原文: &str) -> モジュール宣言の抽出 {
    モジュール宣言の抽出::生成する(Path::new(宣言元).to_path_buf(), 原文)
}

fn 論理の表記(宣言: &モジュール宣言) -> String {
    宣言.論理のモジュールパス().表記().to_string()
}

fn 物理の表記(宣言: &モジュール宣言) -> String {
    宣言.対象の物理ファイル().map(|ファイル| モジュールパス::ファイルのパスから求める(&ファイル).表記().to_string()).unwrap_or_default()
}

#[test]
fn 抽出_path属性が在る宣言と無い宣言の両方を抽出する() {
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 接触;\nmod tests;\n").宣言一覧();
    assert_eq!(宣言一覧.len(), 2);
    assert_eq!(宣言一覧[0].モジュール名, "接触");
    assert_eq!(宣言一覧[0].行番号, 2);
    assert_eq!(宣言一覧[0].置き場の指定, 置き場の指定::属性で明示する("接触.rs".to_string()));
    assert_eq!(宣言一覧[1].モジュール名, "tests");
    assert_eq!(宣言一覧[1].置き場の指定, 置き場の指定::既定の探索に任せる);
}

#[test]
fn 抽出_可視性と他の属性が挟まる形を抽出する() {
    let 原文 = "#[path = \"接触.rs\"]\n#[cfg(test)]\npub(crate) mod 接触;\n#[path = \"シーン.rs\"]\npub mod シーン;\n";
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", 原文).宣言一覧();
    assert_eq!(宣言一覧.len(), 2);
    assert_eq!(宣言一覧[0].置き場の指定, 置き場の指定::属性で明示する("接触.rs".to_string()));
    assert_eq!(宣言一覧[1].置き場の指定, 置き場の指定::属性で明示する("シーン.rs".to_string()));
}

#[test]
fn 抽出_波括弧の中の宣言を本体の形として抽出する() {
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", "mod 接触 {\n    pub struct 位置;\n}\n").宣言一覧();
    assert_eq!(宣言一覧.len(), 1);
    assert_eq!(宣言一覧[0].本体の形, 本体の形::波括弧の中);
    assert_eq!(宣言一覧[0].対象の物理ファイル(), None);
}

#[test]
fn 抽出_宣言に結び付かないpath属性の行を返す() {
    let 対象 = 抽出("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\npub struct 位置;\n");
    assert_eq!(対象.宣言に結び付かないpath属性の行一覧(), vec![1]);
    assert!(対象.宣言一覧().is_empty());
}

#[test]
fn 宣言_lib_rsの日本語モジュールは論理と物理が一致する() {
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 接触;\n").宣言一覧();
    assert_eq!(論理の表記(&宣言一覧[0]), "a::接触");
    assert_eq!(物理の表記(&宣言一覧[0]), "a::接触");
}

#[test]
fn 宣言_日本語ディレクトリの下のモジュールは論理と物理が一致する() {
    let 宣言一覧 = 抽出("crates/a/src/接触.rs", "#[path = \"接触/シーン.rs\"]\nmod シーン;\n").宣言一覧();
    assert_eq!(論理の表記(&宣言一覧[0]), "a::接触::シーン");
    assert_eq!(物理の表記(&宣言一覧[0]), "a::接触::シーン");
}

#[test]
fn 宣言_ディレクトリを束ねるmod_rsを指す形も論理と物理が一致する() {
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", "#[path = \"接触/mod.rs\"]\nmod 接触;\n").宣言一覧();
    assert_eq!(論理の表記(&宣言一覧[0]), "a::接触");
    assert_eq!(物理の表記(&宣言一覧[0]), "a::接触");
}

#[test]
fn 宣言_モジュール名が物理のファイル名と違えば論理と物理が食い違う() {
    let 宣言一覧 = 抽出("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 衝突;\n").宣言一覧();
    assert_eq!(論理の表記(&宣言一覧[0]), "a::衝突");
    assert_eq!(物理の表記(&宣言一覧[0]), "a::接触");
}

#[test]
fn 宣言_前置きが宣言元の置き場と違えば論理と物理が食い違う() {
    let 宣言一覧 = 抽出("crates/a/src/a.rs", "#[path = \"b/接触.rs\"]\nmod 接触;\n").宣言一覧();
    assert_eq!(論理の表記(&宣言一覧[0]), "a::a::接触");
    assert_eq!(物理の表記(&宣言一覧[0]), "a::b::接触");
    assert_eq!(宣言一覧[0].正規形のpath属性の表記(), "a/接触.rs");
}
