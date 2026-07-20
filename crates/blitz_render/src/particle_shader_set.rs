//! コンピュート・頂点・フラグメントのSPIR-Vバイト列を保持する値オブジェクト。
//! GPU粒子トイ(M5波2判断29)のパイプライン2本(コンピュート+グラフィックス)を
//! 生成するための入力。`シェーダー一式`(scene.slang用)と対になる型。

use thiserror::Error;

/// `粒子シェーダー一式::生成する` が失敗したときのエラー。
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum 粒子シェーダー一式エラー {
    /// コンピュート・頂点・フラグメントいずれかのSPIR-Vバイト列が空だった。
    #[error("粒子シェーダーのSPIR-Vバイト列が空だった")]
    空のバイト列,
    /// SPIR-Vは32bit語単位のバイナリ形式のため、バイト長は4の倍数であるべき。
    #[error("粒子シェーダーのSPIR-Vバイト長が4の倍数でなかった: {0}バイト")]
    非4バイト整列(usize),
}

/// 粒子コンピュートパイプライン1本+粒子描画パイプライン1本ぶんのSPIR-Vを保持する値オブジェクト。
#[derive(Debug, Clone)]
pub struct 粒子シェーダー一式 {
    コンピュートspirv: Vec<u8>,
    頂点spirv: Vec<u8>,
    フラグメントspirv: Vec<u8>,
}

impl 粒子シェーダー一式 {
    /// 各バイト列が空でなく4の倍数長であることを検証して生成する。
    pub fn 生成する(
        コンピュートspirv: Vec<u8>,
        頂点spirv: Vec<u8>,
        フラグメントspirv: Vec<u8>,
    ) -> Result<Self, 粒子シェーダー一式エラー> {
        for バイト列 in [&コンピュートspirv, &頂点spirv, &フラグメントspirv] {
            if バイト列.is_empty() {
                return Err(粒子シェーダー一式エラー::空のバイト列);
            }
            if バイト列.len() % 4 != 0 {
                return Err(粒子シェーダー一式エラー::非4バイト整列(バイト列.len()));
            }
        }
        Ok(Self { コンピュートspirv, 頂点spirv, フラグメントspirv })
    }

    pub(crate) fn コンピュートコード(&self) -> &[u8] {
        &self.コンピュートspirv
    }

    pub(crate) fn 頂点コード(&self) -> &[u8] {
        &self.頂点spirv
    }

    pub(crate) fn フラグメントコード(&self) -> &[u8] {
        &self.フラグメントspirv
    }
}
