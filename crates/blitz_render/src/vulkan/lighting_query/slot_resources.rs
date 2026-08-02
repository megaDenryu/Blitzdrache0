//! 1つの進行中フレームスロットが専有する照明問い合わせ資源。触れるのはヘッダの定数バッファ・方向光レコード列・
//! 局所光レコード列の3本と、それらを結んだディスクリプタセットだけである。
//!
//! 不変条件: このスロットへ書けるのは、そのスロットの描画完了フェンスを通過した後だけである。
//! 全スロットで1つのセットとバッファを共有して毎フレーム上書きすると、GPUが読んでいる最中のバッファを
//! 書き換えることになる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。

use ash::vk;

use super::pack::照明問い合わせのバイト列;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::照明問い合わせのバッファ組;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;

/// 1本ぶんのホスト可視バッファ。バッファとメモリは常に組で生き死にする。
pub(super) struct 書き換えバッファ {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 書き換えバッファ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        バイト長: usize,
        用途: vk::BufferUsageFlags,
    ) -> Result<Self, レンダラーエラー> {
        let (buffer, memory) = host_buffer::確保して書き込む(device, メモリプロパティ, &vec![0u8; バイト長], 用途)?;
        Ok(Self { buffer, memory })
    }

    fn 書き込む(&self, device: &ash::Device, バイト列: &[u8]) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.memory, バイト列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}

pub(super) struct スロット資源 {
    pub(super) ヘッダ: 書き換えバッファ,
    pub(super) 方向光列: 書き換えバッファ,
    pub(super) 局所光列: 書き換えバッファ,
    pub(super) セット: vk::DescriptorSet,
}

impl スロット資源 {
    /// ディスクリプタの結び方だけを知るモジュールへ渡す3本のハンドル。
    pub(super) fn バッファ組(&self) -> 照明問い合わせのバッファ組 {
        照明問い合わせのバッファ組 {
            ヘッダ: self.ヘッダ.buffer,
            方向光列: self.方向光列.buffer,
            局所光列: self.局所光列.buffer,
        }
    }

    pub(super) fn 書き込む(
        &self, device: &ash::Device, バイト列: &照明問い合わせのバイト列
    ) -> Result<(), レンダラーエラー> {
        self.ヘッダ.書き込む(device, &バイト列.ヘッダ)?;
        self.方向光列.書き込む(device, &バイト列.方向光列)?;
        self.局所光列.書き込む(device, &バイト列.局所光列)
    }

    /// 注意: ディスクリプタセットの解放はプールの破棄が暗黙に行うため、ここではバッファだけを破棄する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.局所光列.破棄する(device);
        self.方向光列.破棄する(device);
        self.ヘッダ.破棄する(device);
    }
}
