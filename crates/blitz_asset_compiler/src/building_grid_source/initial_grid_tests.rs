//! 道具が作る初期の格子が、保存の検査をそのまま通ることの試験。生成の側と受入の側の一致を機械で確かめる。
//!
//! この試験が守るのは「道具が生成する初期値が道具自身の検証を通ること」であり、道具を開いて建物を1件作った
//! 直後に保存が拒まれるという、人が手で気付くまで見つからない破れをここで止める。
//! 参照: `~/.claude/skills/エディター制作`「生成と検証の対称性」

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::source_fixture::道具が作る初期の格子;

#[test]
fn 道具が作る初期の格子は保存の検査を通る() {
    let 定義 = 道具が作る初期の格子("grid_initial").格子由来の建物定義へ解く().unwrap();
    assert_eq!(定義.識別子().綴り(), "grid_initial");
    assert_eq!(定義.格子().升目の数(), 1);
}

#[test]
fn 道具が作る初期の格子はjsonへ書いて読み戻しても同じ値になる() {
    let ソース = 道具が作る初期の格子("grid_initial");
    let 本文 = serde_json::to_string_pretty(&ソース).unwrap();
    let 読み戻し: super::source::建物の格子ソース = serde_json::from_str(&本文).unwrap();
    assert_eq!(読み戻し, ソース);
}

#[test]
fn 形式版が未対応の格子はファイルの読みが拒む() {
    let mut ソース = 道具が作る初期の格子("grid_initial");
    ソース.形式版 = 99;
    let 本文 = serde_json::to_string(&ソース).unwrap();
    let 一時ファイル = std::env::temp_dir().join("blitzdrache0_未対応版の格子.json");
    std::fs::write(&一時ファイル, 本文).unwrap();
    let 結果 = super::grid_file::建物の格子のファイル::生成する(一時ファイル.clone()).読んで解く();
    std::fs::remove_file(&一時ファイル).unwrap();
    assert!(matches!(
        結果,
        Err(super::error::建物の格子のソースエラー::形式版に対応していない { 版: 99, .. })
    ));
}
