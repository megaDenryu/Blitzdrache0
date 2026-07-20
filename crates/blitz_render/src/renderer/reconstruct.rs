//! スワップチェーン・深度バッファ・HDR中間画像・提示同期の再構築(リサイズ・陳腐化への対応)。
//! パイプラインは動的ビューポート/シザーのため再構築不要（判断8）。
//! フレームごとの同期物（フェンス・取得セマフォ）とコマンドバッファは
//! 再構築時に作り直さない（判断13）。

use ash::vk;

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::vulkan;

impl レンダラー {
    pub(super) fn スワップチェーンを再構築する(&mut self) -> Result<(), レンダラーエラー> {
        // 安全性: 古いスワップチェーン・深度バッファ・HDR/ブルーム画像・提示同期の破棄前にGPU使用完了を待つ。
        unsafe { self.device.device_wait_idle()? };
        self.提示同期.破棄する(&self.device);
        self.深度バッファ.破棄する(&self.device);
        // take()で外してから破棄し、再生成が失敗してもDropが破棄済みハンドルへ触れないようにする。
        if let Some(hdr) = self.hdrターゲット.take() {
            hdr.破棄する(&self.device);
        }
        if let Some(ブルームターゲット) = self.ブルームターゲット.take() {
            ブルームターゲット.破棄する(&self.device);
        }
        self.swapchain.破棄する(&self.device, &self.swapchain_loader);

        self.swapchain = vulkan::swapchain::スワップチェーン::生成する(
            self.physical_device,
            &self.device,
            &self.surface_loader,
            self.surface,
            &self.swapchain_loader,
            self.現在の寸法,
            vk::SwapchainKHR::null(),
        )?;
        // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
        let メモリプロパティ =
            unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };
        self.深度バッファ =
            vulkan::depth::深度バッファ::生成する(&self.device, &メモリプロパティ, self.swapchain.寸法)?;
        // HDR/ブルーム画像はスワップチェーン寸法に連動するため作り直し、ブルーム・トーンマップの
        // ディスクリプタを新ビューへ束縛し直す(判断38・39)。ポスト有効の目印には、破棄されず
        // 残っているトーンマップ一式を使う(hdrターゲットは上でtake済みのため使えない)。
        if self.トーンマップ.is_some() {
            let 新hdr =
                vulkan::hdr_target::HDRターゲット::生成する(&self.device, &メモリプロパティ, self.swapchain.寸法)?;
            let 新ブルーム = match vulkan::bloom_targets::ブルームターゲット::生成する(
                &self.device,
                &メモリプロパティ,
                self.swapchain.寸法,
            ) {
                Ok(一式) => 一式,
                Err(誤り) => {
                    新hdr.破棄する(&self.device);
                    return Err(誤り);
                }
            };
            if let Some(ブルーム) = &self.ブルーム {
                ブルーム.ビューを再束縛する(&self.device, 新hdr.画像ビュー, 新ブルーム.a.画像ビュー, 新ブルーム.b.画像ビュー);
            }
            if let Some(トーンマップ) = &self.トーンマップ {
                トーンマップ.ビューを再束縛する(&self.device, 新hdr.画像ビュー, 新ブルーム.a.画像ビュー);
            }
            self.hdrターゲット = Some(新hdr);
            self.ブルームターゲット = Some(新ブルーム);
        }
        self.提示同期 = vulkan::sync::提示同期::生成する(&self.device, self.swapchain.画像数())?;
        self.再構築が必要 = false;
        Ok(())
    }
}
