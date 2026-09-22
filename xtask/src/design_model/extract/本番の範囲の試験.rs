//! 抽出の全規則が同じ本番の範囲を使うことと、名前の推測をしないことを固定する。
use super::source_group::抽出対象のソース群;
use super::test_support::{原文から結末を組む, 関係の表記一覧};
use std::path::PathBuf;

#[test]
fn 範囲外のクレートはマーカーも関係も供給しない() {
    let 結末 = 原文から結末を組む(&[("crates/blitz_sim/src/entry.rs", "struct A { b: B }\nstruct B;\nimpl M不変データ for A {}\nimpl M不変データ for B {}\npub trait M局所: M状態 {}\n")]);
    assert!(結末.グラフ.関係一覧().is_empty());
    assert!(!結末.グラフ.概念一覧().iter().any(|概念| 概念.識別子().クレート名() == "blitz_sim"));
}

#[test]
fn 試験のモジュールと子孫と直書きの試験型は本番の母集団へ入らない() {
    let 原文一覧 = vec![
        (
            PathBuf::from(format!("crates/blitz_esca/src/lib{}", crate::conform::design_ontology::module_path::ソースのファイルの末尾)),
            "pub mod 本番;\n#[cfg(test)]\n#[path = \"別名/試験.rs\"]\nmod 試験;\n#[cfg(test)]\nstruct 試験型;\n#[cfg(test)]\nimpl Mエンティティ for 試験型 {}\n#[cfg(test)]\nmod 内部 { struct 内部型; impl Mエンティティ for 内部型 {} }\n".to_string(),
        ),
        (PathBuf::from("crates/blitz_esca/src/本番.rs"), "struct 本番型;\nimpl M不変データ for 本番型 {}\n".to_string()),
        (
            PathBuf::from(format!("crates/blitz_esca/src/別名/mod{}", crate::conform::design_ontology::module_path::ソースのファイルの末尾)),
            "mod 子;\nstruct 試験;\nimpl Mエンティティ for 試験 {}\n".to_string(),
        ),
        (PathBuf::from("crates/blitz_esca/src/別名/子.rs"), "struct 子;\nimpl Mエンティティ for 子 {}\n".to_string()),
        (PathBuf::from("crates/blitz_esca/src/未使用.rs"), "struct 未使用;\nimpl Mエンティティ for 未使用 {}\n".to_string()),
    ];
    let 結末 = super::ソース群から抽出する(&抽出対象のソース群::原文一覧から生成する(原文一覧));
    let 関係 = 関係の表記一覧(&結末);
    assert_eq!(関係, vec!["blitz_esca::本番::本番型 下位型である blitz_design::marker::M不変データ"]);
    assert!(!結末.グラフ.概念一覧().iter().any(|概念| 概念.識別子().名前 == "試験型"));
}

#[test]
fn 解釈できない条件付きコンパイルは欠落になる() {
    let 結末 = 原文から結末を組む(&[("crates/blitz_esca/src/entry.rs", "#[cfg(any(test, feature = \"試験\"))]\nstruct 条件型;\nimpl M状態 for 条件型 {}\n")]);
    assert!(結末.グラフ.関係一覧().is_empty());
    assert!(結末.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 本番の子モジュールを辿れなければ欠落になる() {
    let 結末 = 原文から結末を組む(&[("crates/blitz_esca/src/entry.rs", "mod 見つからない;\nstruct A;\nimpl M状態 for A {}\n")]);
    assert!(結末.グラフ.関係一覧().is_empty());
    assert!(結末.抽出できなかった行一覧.iter().any(|行| 行.理由.説明().contains("モジュールの本体")));
}

#[test]
fn 明示した置き場が無いとき別の本体へ読み替えない() {
    let 原文一覧 = vec![
        (
            PathBuf::from(format!("crates/blitz_esca/src/lib{}", crate::conform::design_ontology::module_path::ソースのファイルの末尾)),
            "#[path = \"absent.rs\"]\nmod 子;\n".to_string(),
        ),
        (
            PathBuf::from(format!("crates/blitz_esca/src/absent/mod{}", crate::conform::design_ontology::module_path::ソースのファイルの末尾)),
            "struct 誤った型;\nimpl M状態 for 誤った型 {}\n".to_string(),
        ),
    ];
    let 結末 = super::ソース群から抽出する(&抽出対象のソース群::原文一覧から生成する(原文一覧));
    assert!(結末.グラフ.関係一覧().is_empty());
    assert!(結末.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 試験の属性と項目が同じ行にあれば本番の項目を黙って消さない() {
    let 結末 = 原文から結末を組む(&[("crates/blitz_esca/src/entry.rs", "#[cfg(test)] struct 試験; struct 本番;\nimpl M状態 for 本番 {}\n")]);
    assert!(結末.グラフ.関係一覧().is_empty());
    assert!(結末.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 外部の一括取り込みを別のモジュールの唯一の同名型へ写さない() {
    let 結末 = 原文から結末を組む(&[("crates/blitz_esca/src/holder.rs", "use dependency::*;\nstruct Holder {\n    value: Foo,\n}\n"), ("crates/blitz_esca/src/other.rs", "struct Foo;\n")]);
    assert!(!結末.グラフ.関係一覧().iter().any(|関係| 関係.目的語.識別子().モジュールパス == "blitz_esca::other"));
    assert!(結末.抽出できなかった行一覧.iter().any(|行| 行.理由.説明().contains("Foo")));
}
