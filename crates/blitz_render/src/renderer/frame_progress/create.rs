//! フレーム進行が保つ資源の確保。コマンドプールとコマンドバッファ、フレームごとの同期物をここでまとめて作り、スロット巡回を先頭から始める。
//! 途中で失敗したときに確保済みのコマンドプールを解放する経路をここへ閉じるため、この型が自分の一部を漏らすことはない。

use super::slot_cycle::フレームスロット巡回;
use super::フレーム進行;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

impl フレーム進行 {
    pub(in crate::renderer) fn 生成する(device: &GPUデバイス, キューファミリ添字: u32) -> Result<Self, レンダラーエラー> {
        let (command_pool, command_buffer一覧) = vulkan::commands::生成する(device, キューファミリ添字)?;
        let フレーム同期 = match vulkan::sync::フレーム同期::生成する(device) {
            Ok(値) => 値,
            Err(誤り) => {
                // 安全性: command_poolは直前に確保したばかりで、まだどこにも渡していないためGPUは使用していない。
                unsafe { device.destroy_command_pool(command_pool, None) };
                return Err(誤り);
            }
        };
        Ok(Self {
            command_pool,
            command_buffer一覧,
            フレーム同期,
            巡回: フレームスロット巡回::先頭から始める(),
            見送り累計: 0,
        })
    }
}
