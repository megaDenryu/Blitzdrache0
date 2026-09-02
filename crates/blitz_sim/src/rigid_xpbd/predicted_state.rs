//! 予測の状態: 細分の予測(判断8)が作った配置であり、拘束の補正(判断9)が掛かる唯一の型。反復を終えた値が確定の配置になる。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断8」「判断9」。

use super::correction::姿勢自由度の補正;
use crate::rigid_body::配置;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 予測の状態 {
    配置: 配置,
}

impl 予測の状態 {
    pub(super) fn 生成する(配置: 配置) -> Self {
        Self { 配置 }
    }

    pub fn 配置(&self) -> &配置 {
        &self.配置
    }

    /// 補正を足した予測の状態。並進は重心へ、回転は姿勢へ(正規化は姿勢の型が行う)。
    pub fn 補正を適用する(&self, 補正: &姿勢自由度の補正) -> Self {
        Self {
            配置: 補正.配置へ適用する(&self.配置),
        }
    }
}
