//! `--ibl-step-scan`と`--ibl-step-control`の解析を反証可能な形で固定する。見るのは、指定が起動モードの枝へ入ること、
//! 書き出し先が無い走査を拒むこと、1件も撮らない指定を拒むこと、区切りの数が違う語を拒むこと、
//! 対照が1つの跨ぎを3枚撮ることである。

#![allow(clippy::unwrap_used)]

use super::{起動モード, 起動要求};

fn 解析する(値: &str) -> Result<起動要求, crate::error::起動エラー> {
    let 引数: Vec<String> = vec![
        "--ibl-step-scan".to_string(),
        値.to_string(),
        "--dump-hdr-frame".to_string(),
        "target/ibl_step/shot".to_string(),
    ];
    super::引数を解析する(&引数)
}

/// 圧縮前のHDRの書き出し先が無い走査を拒む。持たないまま走ると絵を1枚も残さず成功して終わる。
#[test]
fn 書き出し先の無い走査を拒む() {
    let 引数: Vec<String> = vec!["--ibl-step-scan".to_string(), "0,1".to_string()];
    assert!(super::引数を解析する(&引数).is_err());
}

fn 走査指定を取り出す(引数: &[String]) -> crate::app::time_of_day::段差走査の指定 {
    let 起動要求::描画実行(設定) = super::引数を解析する(引数).unwrap() else {
        panic!("描画実行にならなかった");
    };
    let 起動モード::段差走査実行 { 走査指定 } = 設定.モード else {
        panic!("段差走査実行にならなかった");
    };
    走査指定
}

#[test]
fn 範囲を段差走査実行の枝へ入れる() {
    let 起動要求::描画実行(設定) = 解析する("81,2").unwrap() else {
        panic!("描画実行にならなかった");
    };
    let 起動モード::段差走査実行 { 走査指定 } = 設定.モード else {
        panic!("段差走査実行にならなかった");
    };
    assert_eq!(走査指定.最初の跨ぎ番号(), 81);
    assert_eq!(走査指定.跨ぎの件数(), 2);
    // 1跨ぎあたり2撮影、1撮影あたり2フレームである。
    assert_eq!(走査指定.総フレーム数(), 8);
}

/// 対照は1つの跨ぎを下側・上側・下側の3枚撮る。
#[test]
fn 対照は一つの跨ぎを三枚撮る() {
    let 引数: Vec<String> = vec![
        "--ibl-step-control".to_string(),
        "97".to_string(),
        "--dump-hdr-frame".to_string(),
        "target/ibl_step/control".to_string(),
    ];
    let 指定 = 走査指定を取り出す(&引数);
    assert_eq!(指定.最初の跨ぎ番号(), 97);
    assert_eq!(指定.跨ぎの件数(), 1);
    assert_eq!(指定.総フレーム数(), 6);
}

#[test]
fn 一本も撮らない指定を拒む() {
    assert!(解析する("0,0").is_err());
}

#[test]
fn 二つの数でない語を拒む() {
    for 値 in ["81", "81,2,3", "a,2"] {
        assert!(解析する(値).is_err(), "{値}");
    }
}
