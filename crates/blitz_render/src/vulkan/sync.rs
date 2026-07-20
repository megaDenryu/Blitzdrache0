//! フレームインフライト1のための同期プリミティブ。
//! フェンス1・取得セマフォ1・提示セマフォ(スワップチェーン画像ごとに1本)を保持する。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) struct 同期プリミティブ {
    pub(crate) 描画完了フェンス: vk::Fence,
    pub(crate) 取得セマフォ: vk::Semaphore,
    pub(crate) 提示セマフォ一覧: Vec<vk::Semaphore>,
}

impl 同期プリミティブ {
    pub(crate) fn 生成する(
        device: &ash::Device,
        スワップチェーン画像数: usize,
    ) -> Result<Self, レンダラーエラー> {
        let フェンス生成情報 =
            vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        // 安全性: deviceは生成済みで有効。
        let 描画完了フェンス = unsafe { device.create_fence(&フェンス生成情報, None)? };

        let セマフォ生成情報 = vk::SemaphoreCreateInfo::default();
        // 安全性: deviceは生成済みで有効。
        let 取得セマフォ = unsafe { device.create_semaphore(&セマフォ生成情報, None)? };

        let mut 提示セマフォ一覧 = Vec::with_capacity(スワップチェーン画像数);
        for _ in 0..スワップチェーン画像数 {
            // 安全性: deviceは生成済みで有効。
            提示セマフォ一覧.push(unsafe { device.create_semaphore(&セマフォ生成情報, None)? });
        }

        Ok(Self {
            描画完了フェンス,
            取得セマフォ,
            提示セマフォ一覧,
        })
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 保持する各ハンドルはSelfが唯一の所有者であり、破棄時点で
        // GPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_fence(self.描画完了フェンス, None);
            device.destroy_semaphore(self.取得セマフォ, None);
            for セマフォ in &self.提示セマフォ一覧 {
                device.destroy_semaphore(*セマフォ, None);
            }
        }
    }
}
