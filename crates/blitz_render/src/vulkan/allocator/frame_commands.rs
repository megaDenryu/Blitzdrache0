//! フレームの記録に使うコマンドプールと、進行中フレーム数ぶんのプライマリコマンドバッファ。
//! プールとバッファ列を1つの資源型にするのは、バッファがプールの破棄で暗黙に解放されるため、
//! 2つを別々に持ち回るとバッファだけが生き残った参照を作れてしまうためである。
//!
//! 参照: _doc/計画/ユビキタス言語.md「資源型」

use ash::vk;

use super::GPU資源の確保係;
use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct フレーム記録のコマンド一式 {
    command_pool: vk::CommandPool,
    command_buffer一覧: [vk::CommandBuffer; 進行中フレーム数],
}

impl フレーム記録のコマンド一式 {
    pub(crate) fn スロットのコマンドバッファ(&self, 配列添字: usize) -> vk::CommandBuffer {
        self.command_buffer一覧[配列添字]
    }

    /// 注意: コマンドバッファはコマンドプールの破棄で暗黙に解放されるため、個別のfree_command_buffersは不要。
    /// 前提: 呼び出し元はGPU側の使用完了を保証している。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: command_poolはSelfが唯一の所有者で、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_command_pool(self.command_pool, None) };
    }
}

impl GPU資源の確保係<'_> {
    pub(crate) fn フレーム記録のコマンド一式を確保する(
        &self,
        キューファミリ添字: u32,
    ) -> Result<フレーム記録のコマンド一式, レンダラーエラー> {
        let プール生成情報 = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(キューファミリ添字);
        // 安全性: deviceは生成済みで有効。キューファミリ添字は選定済みの正当な値。
        let command_pool = unsafe { self.device.create_command_pool(&プール生成情報, None)? };

        match self.進行中フレーム数ぶんのコマンドバッファを割り当てる(command_pool) {
            Ok(command_buffer一覧) => Ok(フレーム記録のコマンド一式 {
                command_pool,
                command_buffer一覧,
            }),
            Err(誤り) => {
                // 安全性: command_poolは直前に生成したばかりで、まだどこにも渡していないためGPUは使用していない。
                unsafe { self.device.destroy_command_pool(command_pool, None) };
                Err(誤り)
            }
        }
    }

    fn 進行中フレーム数ぶんのコマンドバッファを割り当てる(
        &self,
        command_pool: vk::CommandPool,
    ) -> Result<[vk::CommandBuffer; 進行中フレーム数], レンダラーエラー> {
        let 割当数 = u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
        let 割当情報 = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(割当数);
        // 安全性: command_poolは直前に生成済みで、このスコープの唯一の所有者はこの確保係である。
        let command_buffer一覧 = unsafe { self.device.allocate_command_buffers(&割当情報)? };
        let Ok(command_buffer一覧): Result<[vk::CommandBuffer; 進行中フレーム数], _> = command_buffer一覧.try_into() else {
            // command_buffer_count(進行中フレーム数)を要求してVulkanが成功を返したのに
            // 要求数ぴったりで得られないのは、Vulkan実装がその契約を破っている状態であり回復不能。
            panic!("allocate_command_buffersが進行中フレーム数ぴったりのコマンドバッファを返さなかった");
        };
        Ok(command_buffer一覧)
    }
}
