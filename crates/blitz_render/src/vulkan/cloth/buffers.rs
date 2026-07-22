//! 布シミュレーションのバッファ群(判断54)。粒子・前位置・隣接拘束・空間グリッド2本・
//! 布頂点(STORAGE|VERTEX)・インデックス・アタッチ対応と、フレームインフライト2重の
//! 介入キュー・定数UBO(ホスト可視)。生成手順は`create`にある。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::フレームインフライト数;

pub(super) struct 布バッファ {
    pub(super) 粒子: (vk::Buffer, vk::DeviceMemory),
    pub(super) 前位置: (vk::Buffer, vk::DeviceMemory),
    pub(super) 隣接: (vk::Buffer, vk::DeviceMemory),
    pub(super) セルカウント: (vk::Buffer, vk::DeviceMemory),
    pub(super) セル格納: (vk::Buffer, vk::DeviceMemory),
    pub(super) 布頂点: (vk::Buffer, vk::DeviceMemory),
    pub(super) インデックス: (vk::Buffer, vk::DeviceMemory),
    pub(super) アタッチ: (vk::Buffer, vk::DeviceMemory),
    pub(super) 介入一覧: [(vk::Buffer, vk::DeviceMemory); フレームインフライト数],
    pub(super) 定数一覧: [(vk::Buffer, vk::DeviceMemory); フレームインフライト数],
}

pub(super) use create::生成する;

impl 布バッファ {
    /// 前提: 呼び出しはフェンス待ち後(このスロットの前回GPU使用の完了後。判断24と同じ規律)。
    pub(super) fn 介入を書き込む(
        &self, device: &ash::Device, フレーム添字: usize, バイト列: &[u8]
    ) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.介入一覧[フレーム添字].1, バイト列)
    }

    /// 前提: 同上。
    pub(super) fn 定数を書き込む(
        &self, device: &ash::Device, フレーム添字: usize, バイト列: &[u8]
    ) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.定数一覧[フレーム添字].1, バイト列)
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        let 固定一覧 = [
            self.粒子,
            self.前位置,
            self.隣接,
            self.セルカウント,
            self.セル格納,
            self.布頂点,
            self.インデックス,
            self.アタッチ,
        ];
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for &(buffer, memory) in 固定一覧.iter().chain(self.介入一覧.iter()).chain(self.定数一覧.iter()) {
                device.destroy_buffer(buffer, None);
                device.free_memory(memory, None);
            }
        }
    }
}
