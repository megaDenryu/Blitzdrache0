//! 型ごとの分量の台帳の検査が、見出しで認めたGraphiteの生成物の中のimplだけを計測から外し、手で書いたファイルのimplは従来どおり数えることを固定する(Issue #187)。
//! 題材は、手で書いた `地点` とそのimpl、`地点` へimplを足す生成物、別のクレートに在る同じ名前の `地点` の3つである。
//! 生成物を外さないと、生成物の `super::地点` を計測が物理のパスから解けず、2つの `地点` のどちらか絞れない違反になる。

use std::path::PathBuf;

use super::生成物を除いた観測を組む;
use crate::conform::graphiteのコード::Graphiteの生成物の一覧;
use crate::conform::走査した原文の一覧::走査した原文の一覧;
use crate::type_metrics::集計する;

const 宣言のファイル: &str = "crates/仮の経路網/src/仮の図式の宣言.rs";
const 生成物のファイル: &str = "crates/仮の経路網/src/generated/仮の図式の生成物.rs";
const 別のクレートのファイル: &str = "crates/仮の別の網/src/仮の地点.rs";

fn 題材の原文一覧() -> 走査した原文の一覧 {
    let 宣言 = "pub struct 地点;\nimpl 地点 {\n    pub fn 名前を答える(&self) {}\n}\ngraphite::dynamic_graph_schema! {\n    generated = \"generated/仮の図式の生成物.rs\";\n    schema 図式 { node 地点; }\n}\n";
    let 生成物 = "// このファイルは Graphite が生成したため手編集しないこと。\n// 生成元: src/仮の図式の宣言.rs:5\nimpl 図式Insertable for super::地点 {\n    fn 挿入する(&self) {}\n}\n";
    let 別の地点 = "pub struct 地点;\n";
    走査した原文の一覧::生成する(vec![
        (PathBuf::from(宣言のファイル), 宣言.to_string()),
        (PathBuf::from(生成物のファイル), 生成物.to_string()),
        (PathBuf::from(別のクレートのファイル), 別の地点.to_string()),
    ])
}

#[test]
fn 生成物のimplを外しても手書きのimplは計測に入り定義を絞れない違反も出ない() {
    let 原文一覧 = 題材の原文一覧();
    let 生成物 = Graphiteの生成物の一覧::原文一覧から見分ける(&原文一覧);
    assert_eq!(生成物.生成物一覧(), [PathBuf::from(生成物のファイル)], "題材の生成物が見出しと宣言の結び付きで生成物と認められていない");
    let 計測 = 集計する(&生成物を除いた観測を組む(&原文一覧, &生成物));
    assert!(計測.定義の候補を1つに絞れなかった実装ブロック一覧.is_empty(), "生成物のimplが計測に残り、定義を絞れない違反になっている");
    let Some(手書きの地点) = 計測.型ごとの計測一覧.iter().find(|型1件| 型1件.所在.定義ファイルの文字列() == 宣言のファイル) else {
        panic!("手書きの地点が計測に無い");
    };
    assert_eq!(手書きの地点.メソッド総数, 1, "手書きのimplのメソッドが計測に入っていない");
    assert!(!手書きの地点.実装ファイル一覧.contains(&PathBuf::from(生成物のファイル)), "生成物のファイルが実装ファイルに数えられている");
}

#[test]
fn 生成物を外さなければ同じ題材は定義を絞れない違反になる() {
    // 上の試験が空虚に通らないこと(題材そのものが外さないと違反を起こす形であること)を確かめる。
    let 原文一覧 = 題材の原文一覧();
    let 何も外さない = Graphiteの生成物の一覧::原文一覧から見分ける(&走査した原文の一覧::生成する(Vec::new()));
    let 計測 = 集計する(&生成物を除いた観測を組む(&原文一覧, &何も外さない));
    assert!(!計測.定義の候補を1つに絞れなかった実装ブロック一覧.is_empty());
}
