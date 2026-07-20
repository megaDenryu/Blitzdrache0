//! セットアップ時専用の転送実行環境: 一時コマンドバッファをグラフィックスキューへ
//! submitしてfence待ちで完了させるヘルパー(判断2)。頂点/インデックス/テクスチャの
//! ステージング転送、および `シーンを差し替える` の再アップロードが共通で使う。

mod pool;
mod submit;

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) struct 転送実行環境 {
    queue: vk::Queue,
    command_pool: vk::CommandPool,
}

impl 転送実行環境 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        queue: vk::Queue,
        キューファミリ添字: u32,
    ) -> Result<Self, レンダラーエラー> {
        let command_pool = pool::生成する(device, キューファミリ添字)?;
        Ok(Self { queue, command_pool })
    }

    /// `記録`クロージャで積んだ転送コマンドを一時コマンドバッファへ記録し、
    /// グラフィックスキューへsubmitしてfence待ちで完了を保証する。
    pub(crate) fn 一括実行する(
        &self,
        device: &ash::Device,
        記録: impl FnOnce(vk::CommandBuffer),
    ) -> Result<(), レンダラーエラー> {
        submit::一括実行する(device, self.queue, self.command_pool, 記録)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: command_poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_command_pool(self.command_pool, None) };
    }
}
