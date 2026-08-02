//! 材質レコード32バイトのCPU側の並びと、シェーダー宣言との一致の検査。CPUのバイト詰めとslangの原文は
//! コメントでしか結ばれておらず、片方だけを直した食い違いは走らせても色がずれた絵にしかならないため、ここが機械的に見る。

use super::bytes;
use crate::vulkan::material_table::世代内材質レコード;
use crate::vulkan::shader_struct::{シェーダー構造体の並び, 読み取る};

const 材質レコードの原文: &str = include_str!("../../../../../shaders/material_record.slang");

#[test]
fn 材質レコードの各値が決めた開始位置へ並ぶ() {
    let レコード = 世代内材質レコード::試験用に組み立てる([10.0, 11.0, 12.0, 13.0], 20.0, 21.0, [true, false, true], [5, 6, 7]);
    let バイト列 = bytes::バイト列にする(&レコード);
    assert_eq!(バイト列.len(), 48);
    assert_eq!(f32を読む(&バイト列, 0), 10.0);
    assert_eq!(f32を読む(&バイト列, 12), 13.0);
    assert_eq!(f32を読む(&バイト列, 16), 20.0);
    assert_eq!(f32を読む(&バイト列, 20), 21.0);
    assert_eq!(f32を読む(&バイト列, 24), 0.0);
    assert_eq!(u32を読む(&バイト列, 32), 0b101, "特徴ビットは役割の並びのビット集合である");
    assert_eq!(u32を読む(&バイト列, 36), 5);
    assert_eq!(u32を読む(&バイト列, 40), 6);
    assert_eq!(u32を読む(&バイト列, 44), 7);
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
    開始位置を確かめる(&並び, "featureBits", 32);
    開始位置を確かめる(&並び, "baseColorTextureSlot", 36);
    開始位置を確かめる(&並び, "metallicRoughnessTextureSlot", 40);
    開始位置を確かめる(&並び, "normalTextureSlot", 44);
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
