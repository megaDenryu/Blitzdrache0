//! 材質レコード96バイトのCPU側の並びと、シェーダー宣言との一致の検査。CPUのバイト詰めとslangの原文は
//! コメントでしか結ばれておらず、片方だけを直した食い違いは走らせても色がずれた絵にしかならないため、ここが機械的に見る。

use super::bytes;
use crate::vulkan::material_table::世代内材質レコード;
use crate::vulkan::shader_struct::{シェーダー構造体の並び, 読み取る};

const 材質レコードの原文: &str = include_str!("../../../../../shaders/material_record.slang");

fn 検査用のレコード() -> 世代内材質レコード {
    世代内材質レコード::試験用に組み立てる(
        [10.0, 11.0, 12.0, 13.0],
        20.0,
        21.0,
        [true, false, true, true, false, false, false, false],
        [5, 6, 7, 8, 9, 10, 11, 12],
        [30.0, 31.0, 32.0, 33.0],
    )
}

#[test]
fn 材質レコードの各値が決めた開始位置へ並ぶ() {
    let バイト列 = bytes::バイト列にする(&検査用のレコード());
    assert_eq!(バイト列.len(), 96);
    assert_eq!(f32を読む(&バイト列, 0), 10.0);
    assert_eq!(f32を読む(&バイト列, 12), 13.0);
    assert_eq!(f32を読む(&バイト列, 16), 20.0);
    assert_eq!(f32を読む(&バイト列, 20), 21.0);
    assert_eq!(f32を読む(&バイト列, 24), 0.0);
    assert_eq!(f32を読む(&バイト列, 32), 30.0);
    assert_eq!(f32を読む(&バイト列, 44), 33.0);
    assert_eq!(u32を読む(&バイト列, 48), 0b0000_1101, "特徴ビットは役割の並びのビット集合である");
    assert_eq!(u32を読む(&バイト列, 52), 5);
    assert_eq!(u32を読む(&バイト列, 56), 6);
    assert_eq!(u32を読む(&バイト列, 60), 7);
    assert_eq!(u32を読む(&バイト列, 64), 8);
    assert_eq!(u32を読む(&バイト列, 68), 9);
    assert_eq!(u32を読む(&バイト列, 80), 12);
    assert_eq!(u32を読む(&バイト列, 84), 0, "末尾の詰め物は0で埋める");
}

#[test]
fn 材質レコードの宣言がcpu側と同じ並びである() {
    let 並び = match 読み取る(材質レコードの原文, "MaterialRecord") {
        Ok(並び) => 並び,
        Err(誤り) => panic!("MaterialRecordの宣言を読めない: {誤り}"),
    };
    assert_eq!(並び.バイト長, bytes::バイト長);
    開始位置を確かめる(&並び, "baseColorFactor", 0);
    開始位置を確かめる(&並び, "metallicRoughnessFactor", 16);
    開始位置を確かめる(&並び, "surfaceLayerTileScale", 32);
    開始位置を確かめる(&並び, "featureBits", 48);
    開始位置を確かめる(&並び, "baseColorTextureSlot", 52);
    開始位置を確かめる(&並び, "metallicRoughnessTextureSlot", 56);
    開始位置を確かめる(&並び, "normalTextureSlot", 60);
    開始位置を確かめる(&並び, "surfaceLayerWeightTextureSlot", 64);
    開始位置を確かめる(&並び, "surfaceLayerTileTextureSlots", 68);
    開始位置を確かめる(&並び, "padding", 84);
}

fn 開始位置を確かめる(並び: &シェーダー構造体の並び, 名前: &str, 期待する位置: usize) {
    match 並び.開始位置(名前) {
        Some(位置) => assert_eq!(位置, 期待する位置, "{名前}の開始位置がCPU側と食い違う"),
        None => panic!("{名前}がシェーダーの宣言に無い"),
    }
}

fn f32を読む(バイト列: &[u8], 位置: usize) -> f32 {
    f32::from_le_bytes(四バイトを切り出す(バイト列, 位置))
}

fn u32を読む(バイト列: &[u8], 位置: usize) -> u32 {
    u32::from_le_bytes(四バイトを切り出す(バイト列, 位置))
}

fn 四バイトを切り出す(バイト列: &[u8], 位置: usize) -> [u8; 4] {
    match バイト列[位置..位置 + 4].try_into() {
        Ok(配列) => 配列,
        Err(_) => panic!("4バイトの切り出しに失敗した"),
    }
}
