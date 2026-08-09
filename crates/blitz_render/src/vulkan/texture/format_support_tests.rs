//! 用途と格納形式の組から画像形式を選ぶ写像の反証。機材に触れずに写像だけを見る。

#![allow(clippy::unwrap_used)]

use ash::vk;

use super::format_support::vulkan形式を選ぶ;
use crate::error::レンダラーエラー;
use crate::texture_material::{テクスチャ格納形式, テクスチャ用途};

#[test]
fn 非圧縮は色と線形データでsrgbと符号なし正規化に分かれる() {
    let 色 = vulkan形式を選ぶ(テクスチャ用途::色, テクスチャ格納形式::RGBA8).unwrap();
    let 線形 = vulkan形式を選ぶ(テクスチャ用途::線形データ, テクスチャ格納形式::RGBA8).unwrap();
    assert_eq!(色, vk::Format::R8G8B8A8_SRGB);
    assert_eq!(線形, vk::Format::R8G8B8A8_UNORM);
}

/// 初版のBC1は不透明度を1つも運ばないため、不透明度を持つRGBA版でなくRGB版を選ぶ。
#[test]
fn ブロック圧縮の色は不透明度を持たないsrgbの形式を選ぶ() {
    let 形式 = vulkan形式を選ぶ(テクスチャ用途::色, テクスチャ格納形式::BC1).unwrap();
    assert_eq!(形式, vk::Format::BC1_RGB_SRGB_BLOCK);
}

/// 線形データをブロック圧縮で運ぶ組は初版に存在しない。無言で非圧縮へ落とさず拒むことを見る。
#[test]
fn ブロック圧縮と線形データの組を拒む() {
    let 誤り = vulkan形式を選ぶ(テクスチャ用途::線形データ, テクスチャ格納形式::BC1).unwrap_err();
    assert!(matches!(誤り, レンダラーエラー::テクスチャ形式不正(_)), "実際の誤り: {誤り}");
}
