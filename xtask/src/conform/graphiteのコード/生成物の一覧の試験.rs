//! Graphiteの生成物と認める条件を固定する試験。見出しを持つだけでは認めず、生成元の宣言と結び付いたファイルだけを認めることと、
//! 結び付かない見出しを違反として報告することを見る。材料はIssue #187の計測で置いた仮の経路網の宣言と生成物の先頭である。

use std::path::{Path, PathBuf};

use super::super::走査した原文の一覧::走査した原文の一覧;
use super::Graphiteの生成物の一覧;

const 生成元のパス: &str = "crates/blitz_esca/src/仮の経路網.rs";
const 生成物のパス: &str = "crates/blitz_esca/src/generated/経路網.rs";

const 宣言の原文: &str = "#[allow(non_snake_case, dead_code, private_interfaces)]\npub mod 経路網 {\n    include!(\"generated/経路網.rs\");\n}\n\n#[rustfmt::skip]\ngraphite::dynamic_graph_schema! {\n    generated = \"generated/経路網.rs\";\n    schema 経路網 {\n        node 仮の地点;\n    }\n}\n";

fn 見出し付きの原文(生成元の行: &str) -> String {
    format!("// このファイルは Graphite が生成したため手編集しないこと。\n{生成元の行}\n// 再生成: 案内\n\n#[allow(unused_imports)]\nuse super::*;\n/// 1行目\n///\n/// 3行目\npub struct 仮の地点Id(pub String);\n")
}

fn 一覧を組む(原文一覧: &[(&str, String)]) -> Graphiteの生成物の一覧 {
    let 組んだ一覧: Vec<(PathBuf, String)> = 原文一覧.iter().map(|(パス, 原文)| (PathBuf::from(パス), 原文.clone())).collect();
    Graphiteの生成物の一覧::原文一覧から見分ける(&走査した原文の一覧::生成する(組んだ一覧))
}

#[test]
fn 生成元の宣言と結び付いた見出しのファイルを生成物と認める() {
    let 一覧 = 一覧を組む(&[(生成元のパス, 宣言の原文.to_string()), (生成物のパス, 見出し付きの原文("// 生成元: src/仮の経路網.rs:55"))]);
    assert!(一覧.生成物か(Path::new(生成物のパス)));
    assert!(一覧.生成物か(Path::new(r"crates\blitz_esca\src\generated\経路網.rs")), "区切りが逆斜線でも同じファイルと見る");
    assert!(!一覧.生成物か(Path::new(生成元のパス)));
    assert!(一覧.結び付かない見出しの違反一覧().is_empty());
}

#[test]
fn 見出しの無いファイルは生成物と認めず違反にもしない() {
    let 一覧 = 一覧を組む(&[(生成元のパス, 宣言の原文.to_string()), (生成物のパス, "pub struct 手書き;\n".to_string())]);
    assert!(!一覧.生成物か(Path::new(生成物のパス)));
    assert!(一覧.結び付かない見出しの違反一覧().is_empty());
}

#[test]
fn 見出しを偽装した手書きのファイルは生成物と認めず違反にする() {
    let 偽装 = 見出し付きの原文("// 生成元: src/仮の経路網.rs:55");
    let 一覧 = 一覧を組む(&[(生成元のパス, 宣言の原文.to_string()), ("crates/blitz_esca/src/手書き.rs", 偽装.clone()), ("crates/blitz_esca/src/generated/別名.rs", 偽装)]);
    assert!(一覧.生成物一覧().is_empty(), "生成元が生成先に宣言していないファイルは、見出しと置き場が揃っていても認めない");
    assert_eq!(一覧.結び付かない見出しの違反一覧().len(), 2);
}

#[test]
fn 生成元が見つからない見出しと読めない見出しは違反にする() {
    let 一覧 = 一覧を組む(&[
        (生成物のパス, 見出し付きの原文("// 生成元: src/仮の経路網.rs:55")),
        ("crates/blitz_esca/src/generated/読めない見出し.rs", 見出し付きの原文("// 生成元は無い")),
    ]);
    assert!(一覧.生成物一覧().is_empty());
    let 違反一覧 = 一覧.結び付かない見出しの違反一覧();
    assert_eq!(違反一覧.len(), 2);
    assert!(違反一覧.iter().all(|違反| 違反.説明.contains("結び付かない")));
}

#[test]
fn 生成物だけを取り込む波括弧付きのモジュールから取り込む生成物を求める() {
    let 一覧 = 一覧を組む(&[(生成元のパス, 宣言の原文.to_string()), (生成物のパス, 見出し付きの原文("// 生成元: src/仮の経路網.rs:55"))]);
    let 取り込む生成物 = 一覧.波括弧の中で取り込む生成物(Path::new(生成元のパス), 宣言の原文, 2);
    assert!(取り込む生成物.is_some_and(|パス| 一覧.生成物か(&パス)), "宣言のファイルの置き場から数えた取り込み先が生成物である");
    assert_eq!(
        一覧.波括弧の中で取り込む生成物(Path::new(生成元のパス), "pub mod 経路網 {\n    include!(\"generated/手書き.rs\");\n}\n", 1),
        None,
        "生成物と認めていないファイルの取り込みは引かない"
    );
}
