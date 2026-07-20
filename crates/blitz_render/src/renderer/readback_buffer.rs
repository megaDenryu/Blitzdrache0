//! 読み戻しバッファの確保(必要なら再確保)と、読み戻し画像への変換。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::readback_image::読み戻し画像;
use crate::vulkan;

impl レンダラー {
    /// 現在のスワップチェーン寸法ぶんの容量を持つ読み戻しバッファを確保する。
    /// 既存バッファの容量が足りていれば何もしない。
    pub(super) fn 読み戻しバッファを確保する(&mut self) -> Result<(), レンダラーエラー> {
        let 必要バイト数 =
            u64::from(self.swapchain.寸法.width) * u64::from(self.swapchain.寸法.height) * 4;
        let 再確保が必要 = match &self.読み戻しバッファ {
            Some(バッファ) => バッファ.容量バイト数() < 必要バイト数,
            None => true,
        };
        if !再確保が必要 {
            return Ok(());
        }
        if let Some(古い) = self.読み戻しバッファ.take() {
            古い.破棄する(&self.device);
        }
        // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
        let メモリプロパティ =
            unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };
        self.読み戻しバッファ = Some(vulkan::readback::読み戻しバッファ::生成する(
            &self.device,
            &メモリプロパティ,
            必要バイト数,
        )?);
        Ok(())
    }

    /// 提示成功直後に読み戻しバッファをホストから読み、`読み戻し画像` へ変換する。
    pub(super) fn 読み戻しバッファから画像を作る(&self) -> Result<読み戻し画像, レンダラーエラー> {
        let バッファ = self
            .読み戻しバッファ
            .as_ref()
            .unwrap_or_else(|| panic!("提示成功時に読み戻しバッファが未確保だった"));
        vulkan::readback::読み取る(&self.device, バッファ, self.swapchain.寸法, self.swapchain.画像形式)
    }
}
