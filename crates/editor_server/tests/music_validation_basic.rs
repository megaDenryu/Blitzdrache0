//! 楽曲の検証のうち、曲全体に掛かる項目(テンポ・トラックの本数・ミキサー設定)を確かめる。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

mod common;

#[test]
fn 手本の楽曲は検証を通る() {
    common::楽曲の例().検証する().unwrap();
}

#[test]
fn テンポが下限と上限のちょうどなら通る() {
    for 値 in [40, 300] {
        let mut 対象 = common::楽曲の例();
        対象.テンポ = 値;
        対象.検証する().unwrap();
    }
}

#[test]
fn テンポが範囲の外なら拒む() {
    for 値 in [39, 301] {
        let mut 対象 = common::楽曲の例();
        対象.テンポ = 値;
        assert!(対象.検証する().is_err(), "テンポ{値}を通してはならない");
    }
}

#[test]
fn トラック構成が空なら拒む() {
    let mut 対象 = common::楽曲の例();
    対象.トラック構成 = Vec::new();
    assert!(対象.検証する().is_err());
}

#[test]
fn ミキサーの比が0と1のちょうどなら通る() {
    let mut 対象 = common::楽曲の例();
    対象.ミキサー設定.全体の音量 = 0.0;
    対象.ミキサー設定.残響の量 = 1.0;
    対象.ミキサー設定.遅延の量 = 0.0;
    対象.検証する().unwrap();
}

#[test]
fn ミキサーの比が0から1の外なら拒む() {
    let mut 全体の音量が超過 = common::楽曲の例();
    全体の音量が超過.ミキサー設定.全体の音量 = 1.5;
    assert!(全体の音量が超過.検証する().is_err());

    let mut 残響が負 = common::楽曲の例();
    残響が負.ミキサー設定.残響の量 = -0.1;
    assert!(残響が負.検証する().is_err());

    let mut 遅延の量が非数 = common::楽曲の例();
    遅延の量が非数.ミキサー設定.遅延の量 = f64::NAN;
    assert!(遅延の量が非数.検証する().is_err());
}

#[test]
fn 遅延のステップ数は0から32だけを受け入れる() {
    for 値 in [0, 32] {
        let mut 対象 = common::楽曲の例();
        対象.ミキサー設定.遅延のステップ数 = 値;
        対象.検証する().unwrap();
    }
    let mut 超過 = common::楽曲の例();
    超過.ミキサー設定.遅延のステップ数 = 33;
    assert!(超過.検証する().is_err());
}

#[test]
fn 現在の形式版は通り未対応の形式版を拒む() {
    let mut 通る = common::楽曲の例();
    通る.形式版 = editor_server::楽曲の現在の形式版;
    通る.検証する().unwrap();

    let mut 拒む = common::楽曲の例();
    拒む.形式版 = editor_server::楽曲の現在の形式版 + 1;
    assert!(拒む.検証する().is_err());
}

#[test]
fn 楽曲の表示名が空白だけなら拒む() {
    let mut 拒む = common::楽曲の例();
    拒む.表示名 = "\t ".to_string();
    assert!(拒む.検証する().is_err());

    let mut 通る = common::楽曲の例();
    通る.表示名 = "名前のある楽曲".to_string();
    通る.検証する().unwrap();
}
