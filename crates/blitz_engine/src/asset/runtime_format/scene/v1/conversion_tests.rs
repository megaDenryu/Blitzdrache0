//! 版1で焼かれたシーンが最新版の型へ変換されて読めることの検査。版1の書き出しは検査だけが持つため、材料もここで作る。

#![allow(clippy::unwrap_used)]

use super::super::super::{アセット形式版, 実行時アセットを格納する, 実行時アセット種別};
use crate::asset::draw_shape::描画形状;
use crate::asset::runtime_format::実行時形式からシーンを読む;
use crate::asset::runtime_scene_tests::静的シーンを作る;

fn 版1バイト列を作る() -> Vec<u8> {
    let 内容 = super::write::シーン内容を書く(&静的シーンを作る(vec![10, 20, 30, 255], 0.2)).unwrap();
    実行時アセットを格納する(アセット形式版::V1, 実行時アセット種別::シーン, &内容).unwrap()
}

#[test]
fn 版1のシーンを通常メッシュとして最新版へ変換して読む() {
    let 読み取り = 実行時形式からシーンを読む(&版1バイト列を作る()).unwrap();
    assert_eq!(読み取り, 静的シーンを作る(vec![10, 20, 30, 255], 0.2));
    assert!(matches!(読み取り.先頭の描画対象().形状(), 描画形状::通常メッシュ(_)));
}

#[test]
fn 版1と最新版で先頭のヘッダー形式版が異なる() {
    let 版1 = 版1バイト列を作る();
    let 最新版 = crate::asset::シーンを実行時形式へ格納する(&静的シーンを作る(vec![10, 20, 30, 255], 0.2)).unwrap();
    assert_eq!(版1[8..12], アセット形式版::V1.番号().to_le_bytes());
    assert_eq!(最新版[8..12], アセット形式版::V7.番号().to_le_bytes());
}
