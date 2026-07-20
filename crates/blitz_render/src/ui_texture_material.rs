//! 開発用UIテクスチャ用のCPU側データ(RGBA8、ミップ不要)。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断33」。

use thiserror::Error;

/// `UIテクスチャ素材::生成する` が失敗したときのエラー。
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum UIテクスチャ素材エラー {
    /// rgba8のバイト長が`幅 * 高さ * 4`と一致しなかった。
    #[error("UIテクスチャのrgba8バイト長が幅*高さ*4と一致しない: 期待{期待バイト数}, 実際{実バイト数}")]
    バイト長不一致 { 期待バイト数: usize, 実バイト数: usize },
}

/// GPUへ登録するUIテクスチャ1枚ぶんのCPU側データ。ミップは生成しない。
#[derive(Debug, Clone, PartialEq)]
pub struct UIテクスチャ素材 {
    幅: u32,
    高さ: u32,
    rgba8: Vec<u8>,
}

impl UIテクスチャ素材 {
    /// `rgba8.len() == 幅 * 高さ * 4`であることを検証して生成する。
    pub fn 生成する(幅: u32, 高さ: u32, rgba8: Vec<u8>) -> Result<Self, UIテクスチャ素材エラー> {
        let 期待バイト数u64 = u64::from(幅) * u64::from(高さ) * 4;
        let 期待バイト数 = usize::try_from(期待バイト数u64)
            .unwrap_or_else(|_| panic!("UIテクスチャの期待バイト数がusizeに収まらない: {期待バイト数u64}"));
        if rgba8.len() != 期待バイト数 {
            return Err(UIテクスチャ素材エラー::バイト長不一致 {
                期待バイト数,
                実バイト数: rgba8.len(),
            });
        }
        Ok(Self { 幅, 高さ, rgba8 })
    }

    pub(crate) fn 幅(&self) -> u32 {
        self.幅
    }

    pub(crate) fn 高さ(&self) -> u32 {
        self.高さ
    }

    pub(crate) fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }
}
