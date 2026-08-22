//! 台帳が格子のソースを抱えることと、写し取った台帳が後からディスクへ増えた格子を拾わないことの試験。
//!
//! 後者を固定するのは、ソースアセットの書き出しが写し取った台帳だけを読むためである。書き出しの最中に
//! 保存が走ってディスクの格子が増えたとき、カタログと格子の世代が揃わないまま1つのチャンクへ焼かれてはならない。

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::directory::建物の格子の置き場;
use super::ledger::建物の格子の台帳;
use super::source_fixture::{識別子を作る, 道具が作る初期の格子};

fn 試験の置き場を用意する(名前: &str) -> 建物の格子の置き場 {
    let ルート: PathBuf = std::env::temp_dir().join("blitzdrache0_building_grid_ledger").join(名前);
    std::fs::remove_dir_all(&ルート).ok();
    std::fs::create_dir_all(&ルート).unwrap();
    建物の格子の置き場::プロジェクトルートから生成する(&ルート)
}

fn 格子を1件置く(置き場: &建物の格子の置き場, 綴り: &str) {
    置き場
        .建物定義の格子ファイル(&識別子を作る(綴り))
        .書き出す(&道具が作る初期の格子(綴り))
        .unwrap();
}

#[test]
fn 台帳は解けた定義と格子のソースの両方を引ける() {
    let 置き場 = 試験の置き場を用意する("source_and_definition");
    格子を1件置く(&置き場, "grid_house_a");
    let 台帳 = 建物の格子の台帳::置き場から読む(&置き場).unwrap();
    let 識別子 = 識別子を作る("grid_house_a");
    assert_eq!(台帳.識別子で引く(&識別子).unwrap().表示名(), "新しい建物");
    assert_eq!(台帳.格子ソースを識別子で引く(&識別子).unwrap().格子.升目一覧.len(), 1);
}

#[test]
fn 写し取った台帳は後から置き場へ増えた格子を拾わない() {
    let 置き場 = 試験の置き場を用意する("snapshot_is_stable");
    格子を1件置く(&置き場, "grid_house_a");
    let 写し = 建物の格子の台帳::置き場から読む(&置き場).unwrap().clone();

    格子を1件置く(&置き場, "grid_house_b");
    assert_eq!(写し.件数(), 1);
    assert!(写し.格子ソースを識別子で引く(&識別子を作る("grid_house_b")).is_none());
    assert_eq!(建物の格子の台帳::置き場から読む(&置き場).unwrap().件数(), 2);
}
