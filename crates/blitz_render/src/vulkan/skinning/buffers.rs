//! スキニングの4種バッファ(判断44): レスト頂点(読み)・スキン属性(読み)・
//! スキン行列(進行中フレーム2重、ホスト可視で毎フレーム書く)・スキン済み頂点(書き)。
//! 生成手順は`create`にある。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{フレームスロットごとのバッファ, 専用メモリ付きバッファ};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct スキニングバッファ {
    レスト頂点: 専用メモリ付きバッファ,
    属性: 専用メモリ付きバッファ,
    行列一覧: フレームスロットごとのバッファ,
    pub(super) 出力: 専用メモリ付きバッファ,
}

impl スキニングバッファ {
    pub(super) fn レスト頂点buffer(&self) -> vk::Buffer {
        self.レスト頂点.バッファのハンドル()
    }

    pub(super) fn 属性buffer(&self) -> vk::Buffer {
        self.属性.バッファのハンドル()
    }

    pub(super) fn 行列buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.行列一覧.スロットのバッファ(フレーム添字)
    }

    /// 前提: 呼び出しはフェンス待ち後(このスロットの前回GPU使用の完了後。判断24と同じ規律)。
    pub(super) fn 行列を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        行列一覧: &[[f32; 16]],
    ) -> Result<(), レンダラーエラー> {
        let mut バイト列 = Vec::with_capacity(行列一覧.len() * 64);
        for 行列 in 行列一覧 {
            for 成分 in 行列 {
                バイト列.extend_from_slice(&成分.to_le_bytes());
            }
        }
        self.行列一覧.スロットの中身を書き換える(device, フレーム添字, &バイト列)
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for バッファ in [&self.レスト頂点, &self.属性, &self.出力] {
            バッファ.破棄する(device);
        }
        self.行列一覧.破棄する(device);
    }
}
