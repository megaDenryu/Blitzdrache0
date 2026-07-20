//! スワップチェーン画像ごとの提示セマフォ。画像数が変わりうるため
//! スワップチェーン再構築のたびに作り直す。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) struct 提示同期 {
    提示セマフォ一覧: Vec<vk::Semaphore>,
}

impl 提示同期 {
    pub(crate) fn 生成する(device: &ash::Device, 画像数: usize) -> Result<Self, レンダラーエラー> {
        let セマフォ生成情報 = vk::SemaphoreCreateInfo::default();
        let mut 提示セマフォ一覧 = Vec::with_capacity(画像数);
        for _ in 0..画像数 {
            // 安全性: deviceは生成済みで有効。
            提示セマフォ一覧.push(unsafe { device.create_semaphore(&セマフォ生成情報, None)? });
        }
        Ok(Self { 提示セマフォ一覧 })
    }

    pub(crate) fn 提示セマフォ(&self, 画像添字: usize) -> vk::Semaphore {
        self.提示セマフォ一覧[画像添字]
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各セマフォはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for &セマフォ in &self.提示セマフォ一覧 {
                device.destroy_semaphore(セマフォ, None);
            }
        }
    }
}
