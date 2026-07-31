//! フレームごとに多重化する同期物: 描画完了フェンス・取得セマフォ。
//! スワップチェーン再構築では作り直さない（レンダラー生成時の1回だけ生成する）。

use ash::vk;

use super::{フレームスロット添字, 進行中フレーム数};
use crate::error::レンダラーエラー;

pub(crate) struct フレーム同期 {
    描画完了フェンス一覧: [vk::Fence; 進行中フレーム数],
    取得セマフォ一覧: [vk::Semaphore; 進行中フレーム数],
}

impl フレーム同期 {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let フェンス生成情報 = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let セマフォ生成情報 = vk::SemaphoreCreateInfo::default();

        let mut 描画完了フェンス一覧 = [vk::Fence::null(); 進行中フレーム数];
        let mut 取得セマフォ一覧 = [vk::Semaphore::null(); 進行中フレーム数];
        for 添字 in 0..進行中フレーム数 {
            // 安全性: deviceは生成済みで有効。
            描画完了フェンス一覧[添字] = unsafe { device.create_fence(&フェンス生成情報, None)? };
            // 安全性: 同上。
            取得セマフォ一覧[添字] = unsafe { device.create_semaphore(&セマフォ生成情報, None)? };
        }

        Ok(Self {
            描画完了フェンス一覧,
            取得セマフォ一覧,
        })
    }

    pub(crate) fn フェンス(&self, フレーム添字: フレームスロット添字) -> vk::Fence {
        self.描画完了フェンス一覧[フレーム添字.配列添字()]
    }

    pub(crate) fn 取得セマフォ(&self, フレーム添字: フレームスロット添字) -> vk::Semaphore {
        self.取得セマフォ一覧[フレーム添字.配列添字()]
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for &フェンス in &self.描画完了フェンス一覧 {
                device.destroy_fence(フェンス, None);
            }
            for &セマフォ in &self.取得セマフォ一覧 {
                device.destroy_semaphore(セマフォ, None);
            }
        }
    }
}
