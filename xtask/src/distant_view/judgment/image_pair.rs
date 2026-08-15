//! RGBA8とD32の対を同じ寸法の検収画像として読む。
//!
//! 読む段の破れと2枚の寸法の食い違いを別の型で返すのは、前者が採り直しで直る器の破れであり、
//! 後者が突き合わせの前提そのものへ課した判定であるためである。
//! 寸法の行を数の対として読む工程は`size_file`が持つ。

mod size_file;

use std::path::Path;

use super::verdict_names::突き合わせる二枚の絵の寸法;
use crate::acceptance::{判定の破れ, 画像の幅};
use crate::distant_view::error::採取の読み取りの破れ;

pub(super) struct 検収画像 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) 色: Vec<u8>,
    pub(super) 深度: Vec<f32>,
}

impl 検収画像 {
    pub(super) fn 読む(置き場: &Path, 名前: &str) -> Result<Self, 採取の読み取りの破れ> {
        let 寸法のパス = 置き場.join(format!("{名前}_color.size"));
        let (幅, 高さ) = size_file::幅と高さを読む(&寸法のパス)?;
        let 色 = size_file::バイト列を読む(&置き場.join(format!("{名前}_color.raw")))?;
        let 深度バイト = size_file::バイト列を読む(&置き場.join(format!("{名前}_depth.depth32")))?;
        let 画素数 = 幅.checked_mul(高さ).ok_or(採取の読み取りの破れ::画素数が桁あふれした {
            パス: 寸法のパス, 幅, 高さ
        })?;
        if 色.len() != 画素数 * 4 || 深度バイト.len() != 画素数 * 4 {
            return Err(採取の読み取りの破れ::寸法とバイト長が食い違う {
                名前: 名前.to_string(),
                画素数,
                色のバイト長: 色.len(),
                深度のバイト長: 深度バイト.len(),
            });
        }
        let 深度 = 深度バイト
            .chunks_exact(4)
            .map(|値| f32::from_le_bytes([値[0], 値[1], 値[2], 値[3]]))
            .collect();
        Ok(Self { 幅, 高さ, 色, 深度 })
    }

    pub(super) fn 同じ寸法を課す(&self, 相手: &Self) -> Result<(), 判定の破れ> {
        突き合わせる二枚の絵の寸法.綴りの一致を課す(&相手.寸法の綴り(), &self.寸法の綴り())
    }

    pub(super) fn 画素数(&self) -> usize {
        self.幅 * self.高さ
    }
    pub(super) fn 画像の幅(&self) -> 画像の幅 {
        画像の幅::生成する(self.幅)
    }
    pub(super) fn 色画素(&self, 添字: usize) -> &[u8] {
        &self.色[添字 * 4..添字 * 4 + 4]
    }
    pub(super) fn 深度画素(&self, 添字: usize) -> [u8; 4] {
        self.深度[添字].to_le_bytes()
    }
    pub(super) fn 非有限深度数(&self) -> usize {
        self.深度.iter().filter(|値| !値.is_finite()).count()
    }

    fn 寸法の綴り(&self) -> String {
        format!("幅{}・高さ{}", self.幅, self.高さ)
    }
}
