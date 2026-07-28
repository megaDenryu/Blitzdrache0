//! 1つの描画対象が可視ID列をどこから読むかを表す判別共用体と、その確保。
//! 触れるのは自分が確保したバッファだけであり、ディスクリプタが指す参照を返す読み取りと、そのフレームの書き込みに閉じる。
//!
//! 個体が1体だけの対象で専用バッファを確保しないのは、束が共有する「値0だけの列」を指せば足りるためである。
//! 全対象へフレームインフライト数ぶんのストレージバッファを足すと、既存の描画対象のGPU確保数が対象ごとに増える
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「レンダラーの資源配置」)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::visible_instance_selection::可視ID列エラー;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::{可視ID列バッファ, 可視ID列参照};

pub(super) enum 可視ID列の出どころ {
    /// 個体が1体だけの対象。束が共有する値0だけの列を読むため、可視判定の対象にならない。
    束の単一個体列,
    専用バッファ(可視ID列バッファ),
}

impl 可視ID列の出どころ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        個体数: u32,
    ) -> Result<Self, レンダラーエラー> {
        if 個体数 <= 1 {
            return Ok(Self::束の単一個体列);
        }
        Ok(Self::専用バッファ(可視ID列バッファ::生成する(
            device,
            メモリプロパティ,
            個体数,
        )?))
    }

    pub(super) fn 参照(&self, 束の単一個体列: &可視ID列バッファ) -> 可視ID列参照 {
        match self {
            Self::束の単一個体列 => 束の単一個体列.参照(),
            Self::専用バッファ(バッファ) => バッファ.参照(),
        }
    }

    /// 個体が1体だけの対象へ可視ID列が与えられたら型付きエラーにする。束の共有列は全対象が読むため、
    /// 1つの対象の都合で書き換えると他の対象まで壊れる。
    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        可視id列: &[u32],
    ) -> Result<(), レンダラーエラー> {
        match self {
            Self::束の単一個体列 => Err(可視ID列エラー::書き込み先なし.into()),
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
