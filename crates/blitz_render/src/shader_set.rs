//! 頂点・画素段のSPIR-Vバイト列を保持する値オブジェクトと、エンジンが受け取ったSPIR-Vへの加工。
//! 加工は現在1つだけあり、シーンの頂点段の位置の組み込み出力へ不変装飾を付ける工程(`position_invariant`)である。
//!
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断10」。

mod position_invariant;

pub use position_invariant::{位置の不変装飾を付ける, 位置の不変装飾エラー};

use thiserror::Error;

/// `シェーダー一式::生成する` が失敗したときのエラー。
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum シェーダー一式エラー {
    /// 頂点・画素段いずれかのSPIR-Vバイト列が空だった。
    #[error("シェーダーのSPIR-Vバイト列が空だった")]
    空のバイト列,
    /// SPIR-Vは32bit語単位のバイナリ形式のため、バイト長は4の倍数であるべき。
    #[error("シェーダーのSPIR-Vバイト長が4の倍数でなかった: {0}バイト")]
    非4バイト整列(usize),

    /// 頂点段へ位置の不変装飾を付けられなかった。深度プリパスと色パスが同じ位置を出す根拠が立たないため、生成を通さない。
    #[error("頂点段へ位置の不変装飾を付けられなかった: {0}")]
    位置の不変装飾を付けられない(#[from] 位置の不変装飾エラー),
}

/// グラフィックスパイプライン1本ぶんの頂点・画素段SPIR-Vを保持する値オブジェクト。
#[derive(Debug, Clone)]
pub struct シェーダー一式 {
    頂点spirv: Vec<u8>,
    画素段spirv: Vec<u8>,
}

impl シェーダー一式 {
    /// 各バイト列が空でなく4の倍数長であることを検証して生成する。
    pub fn 生成する(頂点spirv: Vec<u8>, 画素段spirv: Vec<u8>) -> Result<Self, シェーダー一式エラー> {
        for バイト列 in [&頂点spirv, &画素段spirv] {
            if バイト列.is_empty() {
                return Err(シェーダー一式エラー::空のバイト列);
            }
            if バイト列.len() % 4 != 0 {
                return Err(シェーダー一式エラー::非4バイト整列(バイト列.len()));
            }
        }
        Ok(Self { 頂点spirv, 画素段spirv })
    }

    pub(crate) fn 頂点コード(&self) -> &[u8] {
        &self.頂点spirv
    }

    pub(crate) fn 画素段コード(&self) -> &[u8] {
        &self.画素段spirv
    }
}
