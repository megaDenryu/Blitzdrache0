//! 1つの描画対象が可視ID列をどこから読むかを表す判別共用体と、その確保。
//! 触れるのは自分が確保したバッファだけであり、ディスクリプタが指す参照を返す読み取りと、そのフレームの書き込みに閉じる。
//!
//! 個体が1体だけの対象で専用バッファを確保しないのは、束が共有する「値0だけの列」を指せば足りるためである。
//! 全対象へ進行中フレーム数ぶんのストレージバッファを足すと、既存の描画対象のGPU確保数が対象ごとに増える
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「レンダラーの資源配置」)。

use super::shared_single_column;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::{可視ID列バッファ, 可視ID列参照};

pub(super) enum 可視ID列の出どころ {
    /// 個体が1体だけの対象。束が共有する値0だけの列を読む。可視ID列は全個体の並べ替えであり、個体が1体なら
    /// 内容は常に値0の1件になるため、専用のバッファを持つ必要がない。
    束の単一個体列,
    専用バッファ(可視ID列バッファ),
}

impl 可視ID列の出どころ {
    pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>, 個体数: u32) -> Result<Self, レンダラーエラー> {
        if 個体数 <= 1 {
            return Ok(Self::束の単一個体列);
        }
        Ok(Self::専用バッファ(可視ID列バッファ::生成する(確保係, 個体数)?))
    }

    pub(super) fn 参照(&self, 束の単一個体列: &可視ID列バッファ) -> 可視ID列参照 {
        match self {
            Self::束の単一個体列 => 束の単一個体列.参照(),
            Self::専用バッファ(バッファ) => バッファ.参照(),
        }
    }

    /// 個体が1体だけの対象は共有列を読むため、書き込みは行わない。共有列の内容と矛盾しない選択(唯一の個体を並べた
    /// 値0だけの列)だけを書き込み省略で受理し、それ以外は型付きエラーにする。判定は`shared_single_column`が持つ。
    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        可視id列: &[u32],
    ) -> Result<(), レンダラーエラー> {
        match self {
            Self::束の単一個体列 => shared_single_column::書き込み要求を受理する(可視id列).map_err(Into::into),
            Self::専用バッファ(バッファ) => バッファ.書き込む(device, フレーム添字, 可視id列),
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        match self {
            Self::束の単一個体列 => {}
            Self::専用バッファ(バッファ) => バッファ.破棄する(device),
        }
    }
}
