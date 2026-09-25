//! 抽出の全規則が同じ本番の範囲を使うことと、名前の推測をしないことを固定する。
use super::test_support::{分類の事実の表記一覧, 原文から設計関係グラフと抽出の欠けを組む, 関係の表記一覧};
use std::path::PathBuf;

#[test]
fn 範囲外のクレートはマーカーも関係も供給しない() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_sim/src/entry.rs", "struct A { b: B }\nstruct B;\nimpl M不変データ for A {}\nimpl M不変データ for B {}\npub trait M局所: M状態 {}\n")]);
    assert!(結果.グラフ.関係一覧().is_empty() && 分類の事実の表記一覧(&結果).is_empty());
    assert!(!結果.グラフ.概念一覧().iter().any(|概念| 概念.識別子().クレート名() == "blitz_sim"));
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
    let 結果 = super::変更の前の抽出の結果との突き合わせ::原文一覧から抽出して突き合わせる(原文一覧);
    assert!(関係の表記一覧(&結果).is_empty(), "{:?}", 関係の表記一覧(&結果));
    assert_eq!(分類の事実の表記一覧(&結果), vec!["blitz_esca::本番::本番型 は blitz_design::marker::M不変データ を実装する"]);
    assert!(!結果.グラフ.概念一覧().iter().any(|概念| 概念.識別子().名前 == "試験型"));
}

#[test]
fn 解釈できない条件付きコンパイルは欠落になる() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/entry.rs", "#[cfg(any(test, feature = \"試験\"))]\nstruct 条件型;\nimpl M状態 for 条件型 {}\n")]);
    assert!(結果.グラフ.関係一覧().is_empty() && 分類の事実の表記一覧(&結果).is_empty());
    assert!(結果.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 本番の子モジュールを辿れなければ欠落になる() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/entry.rs", "mod 見つからない;\nstruct A;\nimpl M状態 for A {}\n")]);
    assert!(結果.グラフ.関係一覧().is_empty() && 分類の事実の表記一覧(&結果).is_empty());
    assert!(結果.抽出できなかった行一覧.iter().any(|行| 行.理由.説明().contains("モジュールの本体")));
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
    let 結果 = super::変更の前の抽出の結果との突き合わせ::原文一覧から抽出して突き合わせる(原文一覧);
    assert!(結果.グラフ.関係一覧().is_empty() && 分類の事実の表記一覧(&結果).is_empty());
    assert!(結果.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 試験の属性と項目が同じ行にあれば本番の項目を黙って消さない() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/entry.rs", "#[cfg(test)] struct 試験; struct 本番;\nimpl M状態 for 本番 {}\n")]);
    assert!(結果.グラフ.関係一覧().is_empty() && 分類の事実の表記一覧(&結果).is_empty());
    assert!(結果.関係を落とした抽出の欠落へ写す().在るか());
}

#[test]
fn 外部の一括取り込みを別のモジュールの唯一の同名型へ写さない() {
    let 結果 = 原文から設計関係グラフと抽出の欠けを組む(&[("crates/blitz_esca/src/holder.rs", "use dependency::*;\nstruct Holder {\n    value: Foo,\n}\n"), ("crates/blitz_esca/src/other.rs", "struct Foo;\n")]);
    assert!(!結果.グラフ.関係一覧().iter().any(|関係| 関係.目的語.識別子().モジュールパス == "blitz_esca::other"));
    assert!(結果.抽出できなかった行一覧.iter().any(|行| 行.理由.説明().contains("Foo")));
}

// Graphiteの生成物を取り込む宣言のファイルと、その生成物の対。生成物の見出しの2行目が名乗る生成元は宣言のファイルである。
fn graphiteの宣言と生成物(生成物の一行目: &str) -> Vec<(PathBuf, String)> {
    let 末尾 = crate::conform::design_ontology::module_path::ソースのファイルの末尾;
    vec![
        (PathBuf::from(format!("crates/blitz_esca/src/lib{末尾}")), "#[path = \"抽出の宣言.rs\"]\npub mod 抽出の宣言;\n".to_string()),
        (
            PathBuf::from("crates/blitz_esca/src/抽出の宣言.rs"),
            "pub struct 仮の地点;\nimpl M不変データ for 仮の地点 {}\n#[allow(non_snake_case)]\npub mod 抽出の網 {\n    include!(\"generated/抽出の網.rs\");\n}\ngraphite::dynamic_graph_schema! {\n    generated = \"generated/抽出の網.rs\";\n    schema 抽出の網 { node 仮の地点; }\n}\n".to_string(),
        ),
        (
            PathBuf::from("crates/blitz_esca/src/generated/抽出の網.rs"),
            format!("{生成物の一行目}\n// 生成元: src/抽出の宣言.rs:7\n// 再生成: 案内\n\npub struct 生成物の型;\nimpl M不変データ for 生成物の型 {{}}\n"),
        ),
    ]
}

#[test]
fn graphiteの生成物を取り込むモジュールは辿らず明示して除外した行として数える() {
    let 結果 = super::変更の前の抽出の結果との突き合わせ::原文一覧から抽出して突き合わせる(graphiteの宣言と生成物("// このファイルは Graphite が生成したため手編集しないこと。"));
    assert!(!結果.関係を落とした抽出の欠落へ写す().在るか(), "生成物の取り込みを関係の欠落として数えない");
    assert!(結果.抽出できなかった行一覧.iter().any(|行| 行.行番号 == 4 && 行.理由.種別の呼び名() == "Graphiteの生成物の取り込みである"));
    assert!(!結果.グラフ.概念一覧().iter().any(|概念| 概念.識別子().名前 == "生成物の型"), "生成物の中身は設計概念の抽出の対象でない");
    assert!(結果.グラフ.概念一覧().iter().any(|概念| 概念.識別子().名前 == "仮の地点"), "宣言のファイルの手書きの型は抽出する");
}

#[test]
fn 見出しの無いファイルを取り込むモジュールは辿れない欠落のまま残す() {
    let 結果 = super::変更の前の抽出の結果との突き合わせ::原文一覧から抽出して突き合わせる(graphiteの宣言と生成物("// 手書きのファイル"));
    assert!(結果.関係を落とした抽出の欠落へ写す().在るか());
    assert!(結果.抽出できなかった行一覧.iter().any(|行| 行.行番号 == 4 && 行.理由.説明().contains("モジュールの本体")));
}
