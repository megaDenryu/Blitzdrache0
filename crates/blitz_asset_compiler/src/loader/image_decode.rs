//! 画像のバイト列をRGBA8へ復号する共通の工程。glTFが埋め込んだ画像と、地表層のタイルとして単独で置かれた
//! 画像ファイルの両方がこの1つを通る。
//!
//! 復号の失敗を呼び出し側の語彙へ畳まないのは、glTFの読み取りと地表層の焼きが別々の失敗の語彙を持ち、
//! どちらの語彙にも属さない「画像として読めなかった」をここで表すためである。

use blitz_engine::テクスチャデータ;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("画像のバイト列をRGBA8へ復号できない: {0}")]
pub struct 画像復号エラー(String);

pub(crate) fn 画像のバイト列をrgba8へ復号する(バイト列: &[u8]) -> Result<テクスチャデータ, 画像復号エラー> {
    let 復号画像 = image::load_from_memory(バイト列)
        .map_err(|誤り| 画像復号エラー(誤り.to_string()))?
        .into_rgba8();
    let (幅, 高さ) = 復号画像.dimensions();
    Ok(テクスチャデータ {
        幅,
        高さ,
        rgba8: 復号画像.into_raw(),
    })
}
