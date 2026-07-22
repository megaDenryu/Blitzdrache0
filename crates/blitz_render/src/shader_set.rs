//! 頂点・フラグメントのSPIR-Vバイト列を保持する値オブジェクト。
//!
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断10」。

use thiserror::Error;

/// `シェーダー一式::生成する` が失敗したときのエラー。
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum シェーダー一式エラー {
    /// 頂点・フラグメントいずれかのSPIR-Vバイト列が空だった。
    #[error("シェーダーのSPIR-Vバイト列が空だった")]
    空のバイト列,
    /// SPIR-Vは32bit語単位のバイナリ形式のため、バイト長は4の倍数であるべき。
    #[error("シェーダーのSPIR-Vバイト長が4の倍数でなかった: {0}バイト")]
    非4バイト整列(usize),
}

/// グラフィックスパイプライン1本ぶんの頂点・フラグメントSPIR-Vを保持する値オブジェクト。
#[derive(Debug, Clone)]
pub struct シェーダー一式 {
    頂点spirv: Vec<u8>,
    フラグメントspirv: Vec<u8>,
}

impl シェーダー一式 {
    /// 各バイト列が空でなく4の倍数長であることを検証して生成する。
    pub fn 生成する(頂点spirv: Vec<u8>, フラグメントspirv: Vec<u8>) -> Result<Self, シェーダー一式エラー> {
        for バイト列 in [&頂点spirv, &フラグメントspirv] {
            if バイト列.is_empty() {
                return Err(シェーダー一式エラー::空のバイト列);
            }
            if バイト列.len() % 4 != 0 {
                return Err(シェーダー一式エラー::非4バイト整列(バイト列.len()));
            }
        }
        Ok(Self {
            頂点spirv,
            フラグメントspirv,
        })
    }

    pub(crate) fn 頂点コード(&self) -> &[u8] {
        &self.頂点spirv
    }

    pub(crate) fn フラグメントコード(&self) -> &[u8] {
        &self.フラグメントspirv
    }
}
