//! 照明問い合わせヘッダをバイト列へ組み立てる工程。受け取るのはヘッダ内容、返すのは
//! shaders/lighting_query.slangのLightingQueryHeaderと同じ並びのバイト列である。
//!
//! 注意: 並びはシェーダー側の宣言順と完全に一致させること。ここを崩すと値化けとして現れる。

use super::header_content::照明問い合わせヘッダ内容;

/// バイト長: uint3個(12) + float1個(4) = 16。4つの4バイト値は同じ16バイト区画に収まるため境界を跨がない。
pub(crate) const バイト長: usize = 16;

pub(crate) fn バイト列にする(内容: &照明問い合わせヘッダ内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    バイト列[0..4].copy_from_slice(&内容.方向光件数.to_le_bytes());
    バイト列[4..8].copy_from_slice(&内容.局所光件数.to_le_bytes());
    バイト列[8..12].copy_from_slice(&真偽をuintにする(内容.照明が有効か).to_le_bytes());
    バイト列[12..16].copy_from_slice(&内容.定数間接近似の係数.to_le_bytes());
    バイト列
}

fn 真偽をuintにする(値: bool) -> u32 {
    if 値 { 1 } else { 0 }
}
