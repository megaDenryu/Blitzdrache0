//! ダンプされた圧縮前のHDR画像を読み込む工程。受け取るのはダンプのベース名、返すのは寸法と単精度の成分列である。
//!
//! 書き出し側は`<ベース名>.hdr32`(RGBA単精度リトルエンディアン連結、行優先)と`<ベース名>.size`(幅 高さ)の
//! 2ファイルへ分けるため、読み手も対で扱う。8ビットの読み手(`crate::raw_image`)と分けるのは、
//! 1画素のバイト数も成分の型も違い、同じ読み手で扱うと長さの検算がどちらの形式のものか決まらないためである。

use std::path::{Path, PathBuf};

/// 1画素あたりの成分数。RGBAの4つである。
pub(super) const 画素あたり成分数: usize = 4;
/// 1成分あたりのバイト数。単精度の4バイトである。
const 成分あたりバイト数: usize = 4;

pub(super) struct HDR画像 {
    pub(super) 画素数: usize,
    pub(super) 成分列: Vec<f32>,
}

pub(super) fn 読み込む(ダンプ先: &Path) -> Result<HDR画像, String> {
    let 寸法パス = PathBuf::from(ダンプ先).with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法パス).map_err(|誤り| format!("{}を読めない: {誤り}", 寸法パス.display()))?;
    let mut 要素 = 寸法.split_whitespace();
    let 幅 = 数値を読む(要素.next())?;
    let 高さ = 数値を読む(要素.next())?;
    let 画像パス = PathBuf::from(ダンプ先).with_extension("hdr32");
    let バイト列 = std::fs::read(&画像パス).map_err(|誤り| format!("{}を読めない: {誤り}", 画像パス.display()))?;
    let 期待バイト数 = 幅 * 高さ * 画素あたり成分数 * 成分あたりバイト数;
    if バイト列.len() != 期待バイト数 {
        return Err(format!("寸法とHDRバイト長が違う: {}と{}", 期待バイト数, バイト列.len()));
    }
    Ok(HDR画像 {
        画素数: 幅 * 高さ,
        成分列: 単精度へ開く(&バイト列),
    })
}

fn 単精度へ開く(バイト列: &[u8]) -> Vec<f32> {
    バイト列
        .chunks_exact(成分あたりバイト数)
        .map(|語| f32::from_le_bytes([語[0], 語[1], 語[2], 語[3]]))
        .collect()
}

fn 数値を読む(語: Option<&str>) -> Result<usize, String> {
    語.ok_or_else(|| "HDR画像の寸法が足りない".to_string())?
        .parse()
        .map_err(|誤り| format!("HDR画像の寸法を数として読めない: {誤り}"))
}
