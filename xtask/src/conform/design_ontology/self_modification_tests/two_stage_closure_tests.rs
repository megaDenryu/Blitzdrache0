//! 名前の閉包を2段で求めることの試験。第1段(同じ型を指す名前)は `use` の別名と項目の型の別名の双方向と関連型の片方向で閉じ、第2段(その型を包む名前)は右辺に閉じた名前を含む別名への片方向で閉じ、
//! 第2段の名前から右辺の最後の名前(`Vec`)へ辿らないことと、結果が辿る順序によらないことを固定する。関連型を片方向の辺として残すことで、型引数の中の射影を通した実装が検査へ届くことも固定する。
//! 走査範囲の実物の閉包の件数も固定する。件数が変わったら、増えた名前が閉包へ入った経路を確かめてから期待値を改める。

use std::path::PathBuf;

use super::super::super::source_lexing::コードだけの行一覧;
use super::super::marker_name_closure::名前の言い換えの辺一覧;
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};
use crate::file_scan;

fn 閉包(ソース一覧: &[(PathBuf, Vec<String>)], 種: &str) -> Vec<String> {
    名前の言い換えの辺一覧::全ソースから組む(ソース一覧).閉包(種)
}

#[test]
fn 別名で繋がった同じ型の名前は包む別名の宣言の順序によらず閉包へ入る() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 本体;\nimpl M不変データ for 甲 {}\nuse crate::y::乙 as 甲;\n");
    let 乙 = ソース("crates/a/src/y.rs", "pub type 乙 = crate::x::本体;\n");
    let 包み = ソース("crates/a/src/z.rs", "pub type 乙 = Vec<甲>;\n");
    for ソース一覧 in [vec![甲.clone(), 乙.clone(), 包み.clone()], vec![包み, 乙, 甲]] {
        assert!(閉包(&ソース一覧, "甲").contains(&"本体".to_string()), "{:?}", 閉包(&ソース一覧, "甲"));
    }
}

#[test]
fn 包む別名は閉包へ入り包む型の名前とそれを包む無関係な別名は入らない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\npub type 包み = Vec<規則>;\npub type 更に包み = Option<包み>;\npub type 無関係 = Vec<u8>;\npub type 関数 = fn(u8) -> Vec<u8>;\n",
    )];
    assert_eq!(閉包(&ソース一覧, "規則"), vec!["包み".to_string(), "更に包み".to_string(), "規則".to_string()]);
}

#[test]
fn 関連型は右辺の型から関連型の名前への片方向だけを辿る() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "impl 領域 for 甲 {\n    type 状態 = 規則;\n}\nimpl 領域 for 乙 {\n    type 状態 = 他;\n}\n")];
    assert_eq!(閉包(&ソース一覧, "規則"), vec!["状態".to_string(), "規則".to_string()]);
    assert_eq!(閉包(&ソース一覧, "他"), vec!["他".to_string(), "状態".to_string()]);
}

#[test]
fn 型引数の中の射影を対象に書いた実装と別名は検査へ届く() {
    let 定義 = "pub struct 規則(u8);\nimpl M不変データ for 規則 {}\npub trait 領域 {\n    type 状態;\n}\nimpl 領域 for 甲 {\n    type 状態 = 規則;\n}\n";
    for 実装 in [
        "impl 変更 for Vec<<甲 as 領域>::状態> {\n    fn 変える(&mut self) {}\n}\n",
        "pub type 包み<T> = Vec<<T as 領域>::状態>;\nimpl 変更 for 包み<甲> {\n    fn 変える(&mut self) {}\n}\n",
    ] {
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &format!("{定義}{実装}"))]);
        assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{実装}: {説明一覧:?}");
    }
}

#[test]
fn 走査範囲の実物の閉包の件数() {
    let ルート = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("crates");
    let Some(ルートの表記) = ルート.to_str() else {
        panic!("走査のルートのパスを文字列として読めない: 不変条件「リポジトリのパスはUTF-8である」が破れた");
    };
    let パス一覧 = file_scan::対象ファイル一覧を集める(&[ルートの表記], &["rs"]).unwrap_or_else(|破れ| panic!("実物のcratesを走査できなかった: {破れ}"));
    let ソース一覧: Vec<(PathBuf, Vec<String>)> = パス一覧
        .into_iter()
        .filter(|パス| パス.components().any(|部品| 部品.as_os_str() == "src"))
        .map(|パス| {
            let 原文 = std::fs::read_to_string(&パス).unwrap_or_else(|誤り| panic!("実物のソースを読めなかった: {} {誤り}", パス.display()));
            (パス, コードだけの行一覧(&原文))
        })
        .collect();
    assert_eq!(閉包(&ソース一覧, "旅行者の現在地"), vec!["旅行者の現在地".to_string(), "状態".to_string(), "遷移の関数ポインタ".to_string()]);
    assert_eq!(閉包(&ソース一覧, "遷移パラメータ"), vec!["遷移の関数ポインタ".to_string(), "遷移パラメータ".to_string()]);
}
