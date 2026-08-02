//! 材質レコード32バイトのCPU側の並びと、シェーダー宣言との一致の検査。CPUのバイト詰めとslangの原文は
//! コメントでしか結ばれておらず、片方だけを直した食い違いは走らせても色がずれた絵にしかならないため、ここが機械的に見る。

use super::bytes;
use super::content::材質レコード内容;
use crate::vulkan::shader_struct::{シェーダー構造体の並び, 読み取る};

const 材質レコードの原文: &str = include_str!("../../../../../shaders/material_record.slang");

#[test]
fn 材質レコードの各値が決めた開始位置へ並ぶ() {
    let 内容 = 材質レコード内容 {
        ベースカラー係数: [10.0, 11.0, 12.0, 13.0],
        金属粗さ係数: [20.0, 21.0],
    };
    let バイト列 = bytes::バイト列にする(&内容);
    assert_eq!(バイト列.len(), 32);
    assert_eq!(f32を読む(&バイト列, 0), 10.0);
    assert_eq!(f32を読む(&バイト列, 12), 13.0);
    assert_eq!(f32を読む(&バイト列, 16), 20.0);
    assert_eq!(f32を読む(&バイト列, 20), 21.0);
    assert_eq!(f32を読む(&バイト列, 24), 0.0);
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
}

fn 開始位置を確かめる(並び: &シェーダー構造体の並び, 名前: &str, 期待する位置: usize) {
    match 並び.開始位置(名前) {
        Some(位置) => assert_eq!(位置, 期待する位置, "{名前}の開始位置がCPU側と食い違う"),
        None => panic!("{名前}がシェーダーの宣言に無い"),
    }
}

fn f32を読む(バイト列: &[u8], 位置: usize) -> f32 {
    let 配列 = match バイト列[位置..位置 + 4].try_into() {
        Ok(配列) => 配列,
        Err(_) => panic!("4バイトの切り出しに失敗した"),
    };
    f32::from_le_bytes(配列)
}
