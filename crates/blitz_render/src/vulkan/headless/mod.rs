//! ウィンドウを持たないGPU環境。インスタンス・物理デバイス・論理デバイス・コンピュートキュー・コマンドプールと、
//! validation層の有効化一式だけを持つ。
//!
//! 提示のためのGPU環境と別に持つのは、あちらがサーフェスとスワップチェーンを不可分に抱えており、
//! ウィンドウの無い場面では組み立てられないためである。こちらは描画も提示もせず、コンピュートの
//! ディスパッチと読み戻しだけを行う。使うのは大気のベイク済み画像の読み戻し検査であり、本番の描画経路は通らない。
//!
//! 注意: 破棄の順序はコマンドプール→論理デバイス→メッセンジャー→インスタンスである。この順序をこの型の内部に閉じる。
//! 注意: 検証カウンタはインスタンス破棄より後に読む。破棄そのものが発する指摘も件数へ含めるためである。

mod create;
mod one_shot;
mod select;
mod validation;

use ash::vk;

use crate::validation_counter::{検証カウンタ, 検証層の状況};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct ヘッドレスGPU環境 {
    // 注意: 値としては読まれないが、破棄まで保持し続けることに意味がある。
    // ash::Entryを破棄するとVulkanローダーがアンロードされ得るため、instance/deviceより先に破棄してはならない。
    #[allow(dead_code)]
    entry: ash::Entry,
    検証: validation::検証つきインスタンス,
    検証カウンタ: 検証カウンタ,
    physical_device: vk::PhysicalDevice,
    device: GPUデバイス,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
}

impl ヘッドレスGPU環境 {
    pub(crate) fn device(&self) -> &GPUデバイス {
        &self.device
    }

    /// validation層を有効にできたかどうか。呼び出し元はこの値を報告へ出す義務を持つ。
    pub(crate) fn 検証層の状況(&self) -> 検証層の状況 {
        self.検証.状況
    }

    /// 指摘の件数を数えるカウンタ。破棄より後に読むため、破棄する前に複製を持ち出す。
    pub(crate) fn 検証カウンタを複製する(&self) -> 検証カウンタ {
        self.検証カウンタ.clone()
    }

    pub(crate) fn メモリプロパティを取得する(&self) -> vk::PhysicalDeviceMemoryProperties {
        // 安全性: instance・physical_deviceは生成済みで有効。
        unsafe { self.検証.instance.get_physical_device_memory_properties(self.physical_device) }
    }

    /// コマンドバッファを1本取り、渡された記録を書いて送信し、完了を待つ。
    pub(crate) fn 一度きりで実行する(
        &self,
        記録: impl FnOnce(&ash::Device, vk::CommandBuffer),
    ) -> Result<(), crate::error::レンダラーエラー> {
        one_shot::実行する(&self.device, self.command_pool, self.queue, 記録)
    }

    /// 前提: 呼び出し元はGPUの全作業の完了を待っており(`一度きりで実行する`が送信ごとに待つ)、
    /// この環境で確保した資源をすべて破棄済みである。残っていれば`全メモリ解放を確認する`が止める。
    pub(crate) fn 破棄する(&self) {
        self.device.全メモリ解放を確認する();
        // 安全性: どのハンドルもSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
        }
        self.検証.破棄する();
    }
}
