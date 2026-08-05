//! 撮影1枚ぶんの圧縮前HDRの読み込み。受け取るのはダンプのベース名、返すのは画素ごとの成分列である。
//!
//! `hdr-luminance`の読み手と分けるのは、あちらが画面全体を1つの列として並べ替えるのに対し、ここが
//! 画素の添字で2枚を突き合わせるためである。突き合わせには寸法の一致が要り、その検査を持つ側が違う。

use std::path::{Path, PathBuf};

/// 1画素あたりの成分数。RGBAの4つである。
const 画素あたり成分数: usize = 4;
/// 1成分あたりのバイト数。単精度の4バイトである。
const 成分あたりバイト数: usize = 4;

pub(super) struct HDR画像 {
    幅: usize,
    高さ: usize,
    成分列: Vec<f32>,
}

impl HDR画像 {
    pub(super) fn 画素(&self, 添字: usize) -> &[f32] {
        let 先頭 = 添字 * 画素あたり成分数;
        &self.成分列[先頭..先頭 + 画素あたり成分数]
    }

    pub(super) fn 画素数(&self) -> usize {
        self.幅 * self.高さ
    }

    /// 領域マスクと画素の添字を突き合わせられるかを確かめる。寸法が違えば同じ添字が別の場所を指すため、
    /// 段差を測る前にここで落とす。
    pub(super) fn 寸法が合うか(&self, 相手: &Self) -> Result<(), String> {
        if self.幅 == 相手.幅 && self.高さ == 相手.高さ {
            return Ok(());
        }
        Err(format!("対の2枚の寸法が違う: {}x{}と{}x{}", self.幅, self.高さ, 相手.幅, 相手.高さ))
    }
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
        幅,
        高さ,
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
