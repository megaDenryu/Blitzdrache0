//! テクスチャデータ: デコード済みで常にRGBA8へ正規化した画像。

/// 幅・高さとRGBA8のバイト列（1ピクセル4バイト、行優先）。
#[derive(Debug, Clone, PartialEq)]
pub struct テクスチャデータ {
    pub 幅: u32,
    pub 高さ: u32,
    pub rgba8: Vec<u8>,
}
