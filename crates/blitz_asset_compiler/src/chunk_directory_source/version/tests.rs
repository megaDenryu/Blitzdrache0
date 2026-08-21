//! ソース目録の版ごとの宣言から最新版へ変換する全分岐を固定する。

use super::最新の一辺へ変換する;
use crate::error::チャンク目録コンパイルエラー;

#[test]
fn 版一の二欄は百メートルへ変換する() {
    let Ok(一辺) = 最新の一辺へ変換する("blitz_chunk_directory 1") else {
        panic!("版1を変換できなかった");
    };
    assert_eq!(一辺.f32値(), 100.0);
}

#[test]
fn 版二の正常な一辺を読む() {
    let Ok(一辺) = 最新の一辺へ変換する("blitz_chunk_directory 2 256") else {
        panic!("版2を変換できなかった");
    };
    assert_eq!(一辺.f32値(), 256.0);
}

#[test]
fn 版二の一辺欄欠落を拒む() {
    assert!(matches!(
        最新の一辺へ変換する("blitz_chunk_directory 2"),
        Err(チャンク目録コンパイルエラー::形式宣言不正(_))
    ));
}

#[test]
fn 版二の非数値を拒む() {
    assert!(matches!(
        最新の一辺へ変換する("blitz_chunk_directory 2 abc"),
        Err(チャンク目録コンパイルエラー::形式宣言不正(_))
    ));
}

#[test]
fn 版二の負値を拒む() {
    assert!(matches!(
        最新の一辺へ変換する("blitz_chunk_directory 2 -1"),
        Err(チャンク目録コンパイルエラー::チャンク一辺不正(_))
    ));
}

#[test]
fn 版二の無限大を拒む() {
    assert!(matches!(
        最新の一辺へ変換する("blitz_chunk_directory 2 inf"),
        Err(チャンク目録コンパイルエラー::チャンク一辺不正(_))
    ));
}

#[test]
fn 未対応の版三を拒む() {
    assert!(matches!(
        最新の一辺へ変換する("blitz_chunk_directory 3"),
        Err(チャンク目録コンパイルエラー::未対応版(_))
    ));
}

#[test]
fn 形式名違いを拒む() {
    assert!(matches!(
        最新の一辺へ変換する("other 2 256"),
        Err(チャンク目録コンパイルエラー::形式宣言不正(_))
    ));
}

#[test]
fn 空行を拒む() {
    assert!(matches!(最新の一辺へ変換する(""), Err(チャンク目録コンパイルエラー::形式宣言不正(_))));
}
