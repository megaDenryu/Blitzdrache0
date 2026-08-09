//! テクスチャの格納形式への符号化。BC1のブロック圧縮と、オフラインで焼く縮小段の生成を持つ。
//! どちらもGPUに1度も触れない純関数の層であり、実行時形式への配線は持たない。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`

mod block_compression;
mod level_pixels;
mod mip_chain;
mod pixel_color;

use blitz_engine::テクスチャデータ;

use crate::error::アセットコンパイルエラー;

pub use block_compression::rgba8の縮小段をbc1のバイト列へ符号化する;
pub use mip_chain::{srgbの色として縦横を半分に縮める, srgbの色として縮小段の連なりを作る};

/// 原寸のsRGBの色から全ての縮小段を焼き、段0から順に各段をBC1のバイト列へ符号化する。
/// ブロック圧縮の格納は全段をファイルが持つため、実行時の縮小段の生成を1度も通らない。
pub fn srgbの色の全段をbc1のバイト列へ符号化する(
    原寸: &テクスチャデータ,
) -> Result<Vec<Vec<u8>>, アセットコンパイルエラー> {
    srgbの色として縮小段の連なりを作る(原寸)?
        .iter()
        .map(rgba8の縮小段をbc1のバイト列へ符号化する)
        .collect()
}
