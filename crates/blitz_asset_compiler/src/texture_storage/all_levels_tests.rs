//! 縮小段の生成とBC1の符号化をつないだ入口の検査。段の本数と段ごとのバイト数が
//! エンジンの格納バイト数の算術と一致することを固定する。

#![allow(clippy::unwrap_used)]

use blitz_engine::texture_storage::テクスチャ格納形式;
use blitz_engine::テクスチャデータ;

use super::srgbの色の全段をbc1のバイト列へ符号化する;

#[test]
fn 全段のバイト数が段ごとの算術と一致する() {
    let 段 = テクスチャデータ {
        幅: 8,
        高さ: 8,
        rgba8: [30, 160, 220, 255].repeat(64),
    };
    let 全段 = srgbの色の全段をbc1のバイト列へ符号化する(&段).unwrap();
    let 長さ一覧: Vec<usize> = 全段.iter().map(Vec::len).collect();
    let 期待一覧: Vec<usize> = [(8, 8), (4, 4), (2, 2), (1, 1)]
        .into_iter()
        .map(|(幅, 高さ)| usize::try_from(テクスチャ格納形式::BC1.縮小段の格納バイト数を求める(幅, 高さ).unwrap()).unwrap())
        .collect();
    assert_eq!(長さ一覧, 期待一覧);
}

#[test]
fn 零の寸法のテクスチャを拒む() {
    let 段 = テクスチャデータ {
        幅: 0,
        高さ: 4,
        rgba8: Vec::new(),
    };
    assert!(srgbの色の全段をbc1のバイト列へ符号化する(&段).is_err());
}
