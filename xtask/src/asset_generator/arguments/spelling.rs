//! 役割の型を、生成器へ渡す生の綴りへ写す変換。所有するのは値ごとの綴りだけであり、並べ方は親が持つ。
//!
//! 綴りは生成器側にも同じものがある。方針は
//! `crates/blitz_asset_compiler/examples/compile_assets/texture_policy_argument.rs`が持ち、食い違えば生成器が
//! 「未知のテクスチャ格納方針である」で失敗する。種は10進の非負整数であり、生成器が同じ形で読み戻す。

use blitz_asset_compiler::{テクスチャ格納方針, マップ生成の乱数の種};

const 全てRGBA8の綴り: &str = "all_rgba8";
const ベースカラーのブロック圧縮の綴り: &str = "block_compressed_base_color";

pub(super) fn 方針の綴り(方針: テクスチャ格納方針) -> String {
    match 方針 {
        テクスチャ格納方針::全てRGBA8 => 全てRGBA8の綴り.to_string(),
        テクスチャ格納方針::ベースカラーのブロック圧縮 => ベースカラーのブロック圧縮の綴り.to_string(),
    }
}

pub(super) fn 種の綴り(種: マップ生成の乱数の種) -> String {
    種.値().to_string()
}
