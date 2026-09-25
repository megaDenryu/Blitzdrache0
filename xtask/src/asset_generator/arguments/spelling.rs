//! 役割の型を、生成器へ渡す生の文字列へ写す変換。所有するのは値ごとの文字列だけであり、並べ方は親が持つ。
//!
//! 文字列は生成器側にも同じものがある。方針は
//! `crates/blitz_asset_compiler/examples/compile_assets/texture_policy_argument.rs`が持ち、食い違えば生成器が
//! 「未知のテクスチャ格納の方式である」で失敗する。種は10進の非負整数であり、生成器が同じ形で読み戻す。

use blitz_asset_compiler::{テクスチャ格納の方式, マップ生成の乱数の種, 世界の広がり};

const 全てRGBA8の文字列: &str = "all_rgba8";
const ベースカラーのブロック圧縮の文字列: &str = "block_compressed_base_color";

pub(super) fn 方針の文字列(方針: テクスチャ格納の方式) -> String {
    match 方針 {
        テクスチャ格納の方式::全てRGBA8 => 全てRGBA8の文字列.to_string(),
        テクスチャ格納の方式::ベースカラーのブロック圧縮 => ベースカラーのブロック圧縮の文字列.to_string(),
    }
}

pub(super) fn 種の文字列(種: マップ生成の乱数の種) -> String {
    種.値().to_string()
}

pub(super) fn 東西チャンク数の文字列(広がり: 世界の広がり) -> String {
    広がり.東西チャンク数().to_string()
}

pub(super) fn 南北チャンク数の文字列(広がり: 世界の広がり) -> String {
    広がり.南北チャンク数().to_string()
}
