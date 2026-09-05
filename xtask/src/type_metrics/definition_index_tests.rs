//! implブロックを定義へ引き当てる判断の試験。証明できない帰属を推測しないことを固定する。
//!
//! 反証の要は、同じ名前の定義が2つあり、implが物理的に近い方でなく経路の指す方に属する形である。
//! ファイルやディレクトリの近さで選ぶ実装なら近い方へ誤って加算するため、この形が近さの規則の混入を止める。

use std::path::{Path, PathBuf};

use super::attribution_input::引き当ての材料;
use super::declaration_amount::宣言の分量;
use super::definition_index::定義の索引;
use super::file_observation::ファイルの観測;
use super::impl_attribution::実装ブロックの引き当て;
use super::import_index::取り込みの索引;
use super::observation::観測;
use super::type_path::自己型の経路;

fn 設定の定義があるファイル(パス: &str) -> ファイルの観測 {
    ファイルの観測 {
        パス: PathBuf::from(パス),
        観測一覧: vec![観測::型定義 {
            型名: "設定".to_string(),
            分量: 宣言の分量::構造体のフィールド数(1),
        }],
        取り込みの索引: 取り込みの索引::ファイルの内容から生成する(""),
    }
}

fn 索引(定義のファイル一覧: &[&str]) -> 定義の索引 {
    let 観測: Vec<ファイルの観測> = 定義のファイル一覧.iter().map(|パス| 設定の定義があるファイル(パス)).collect();
    定義の索引::ファイル別の観測から生成する(&観測)
}

fn 引き当てる(
    索引: &定義の索引, 実装ブロックのファイル: &str, 経路の綴り: &str, 取り込み: &str
) -> 実装ブロックの引き当て {
    let 経路 = 自己型の経路::綴りから生成する(経路の綴り);
    let 取り込みの索引 = 取り込みの索引::ファイルの内容から生成する(取り込み);
    索引.実装ブロックの所在を引き当てる(&引き当ての材料::生成する(
        Path::new(実装ブロックのファイル),
        &経路,
        &取り込みの索引,
    ))
}

fn 決まった所在(引き当て: 実装ブロックの引き当て) -> String {
    match 引き当て {
        実装ブロックの引き当て::所在が決まった(所在) => 所在.to_string(),
        実装ブロックの引き当て::定義の候補を1つに絞れない(実装ブロック) => {
            panic!("絞れなかった: {}", 実装ブロック.候補の綴り())
        }
    }
}

#[test]
fn 定義が走査に無ければ実装ブロックのファイルを所在にする() {
    assert_eq!(決まった所在(引き当てる(&索引(&[]), "a/src/alias.rs", "設定", "")), "a/src/alias.rs::設定");
}

#[test]
fn 同じ名前の定義が1件だけなら離れたファイルのimplでもその定義へ帰属する() {
    let 索引 = 索引(&["a/src/far/def.rs"]);
    assert_eq!(
        決まった所在(引き当てる(&索引, "a/src/near/impl.rs", "設定", "")),
        "a/src/far/def.rs::設定"
    );
}

#[test]
fn 実装対象の経路が指す定義へ帰属しファイルの近さでは選ばない() {
    let 索引 = 索引(&["a/src/near/def.rs", "a/src/far/def.rs"]);
    let 引き当て = 引き当てる(&索引, "a/src/near/impl.rs", "crate::far::def::設定", "");
    assert_eq!(決まった所在(引き当て), "a/src/far/def.rs::設定");
}

#[test]
fn 取り込んだ経路が指す定義へ帰属しファイルの近さでは選ばない() {
    let 索引 = 索引(&["a/src/near/def.rs", "a/src/far/def.rs"]);
    let 引き当て = 引き当てる(&索引, "a/src/near/impl.rs", "設定", "use crate::far::def::設定;\n");
    assert_eq!(決まった所在(引き当て), "a/src/far/def.rs::設定");
}

#[test]
fn 経路が定義の在り処まで書かれていなければ引き当てず候補を返す() {
    let 索引 = 索引(&["a/src/near/def.rs", "a/src/far/def.rs"]);
    let 引き当て = 引き当てる(&索引, "a/src/near/impl.rs", "crate::far::設定", "");
    let 実装ブロックの引き当て::定義の候補を1つに絞れない(実装ブロック) = 引き当て else {
        panic!("経路が定義のファイルを指していないのに1つへ絞った");
    };
    assert_eq!(実装ブロック.候補の綴り(), "a/src/near/def.rs::設定 / a/src/far/def.rs::設定");
}

#[test]
fn 経路も取り込みも無ければ引き当てず候補を返す() {
    let 索引 = 索引(&["a/src/left/def.rs", "a/src/right/def.rs"]);
    let 引き当て = 引き当てる(&索引, "a/src/lib.rs", "設定", "");
    assert!(matches!(引き当て, 実装ブロックの引き当て::定義の候補を1つに絞れない(_)));
}

#[test]
fn 同じファイルに定義があればそのファイルの定義へ帰属する() {
    let 索引 = 索引(&["a/src/near/def.rs", "a/src/far/def.rs"]);
    assert_eq!(
        決まった所在(引き当てる(&索引, "a/src/near/def.rs", "設定", "")),
        "a/src/near/def.rs::設定"
    );
}
