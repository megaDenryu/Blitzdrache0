//! 1つの資源表世代が実際に常駐させるテクスチャの枚数。担当するのは、重複除去後の必要枚数が固定のレイアウト容量へ収まることを、
//! ディスクリプタを1つも作る前に確かめることである。
//!
//! レイアウト容量と別の型にするのは、2つの寿命が違うためである。レイアウト容量はパイプラインと同じ寿命で動かず、
//! この枚数は世代ごとに変わる。収まらない世代は公開せず、型付きの失敗として呼び出し元へ返す。
//! 参照: `crates/blitz_render/src/vulkan/material_table/capacity.rs`

use crate::error::材質資源表エラー;

use super::capacity::テクスチャ表レイアウト容量;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct 世代の常駐枚数 {
    枚数: u32,
}

impl 世代の常駐枚数 {
    pub(in crate::vulkan::material_table) fn 確かめる(
        必要枚数: u32,
        レイアウト容量: テクスチャ表レイアウト容量,
    ) -> Result<Self, 材質資源表エラー> {
        if 必要枚数 > レイアウト容量.枚数() {
            return Err(材質資源表エラー::レイアウト容量超過 {
                常駐枚数: 必要枚数,
                レイアウト容量: レイアウト容量.枚数(),
            });
        }
        Ok(Self { 枚数: 必要枚数 })
    }

    pub(in crate::vulkan::material_table) const fn 枚数(self) -> u32 {
        self.枚数
    }
}
