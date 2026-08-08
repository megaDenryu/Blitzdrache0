//! 合成入力のホスト可視バッファの本数を保つ工程と、確保した本数ぶんをまとめて解放する工程。
//! 受け取るのはバッファとメモリの組の並び、返すのは4本の配列か解放の完了である。
//!
//! 独立した工程にするのは、確保が途中で失敗したときの片付けと、破棄のときの片付けが同じ操作だからである。
//! 2箇所へ書くと、片方だけがメモリの解放を落とした状態が台帳の残数としてしか現れない。

use ash::vk;

use crate::vulkan::tracked_device::GPUデバイス;

/// 安全性: バッファ・memoryは呼び出し元が唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを保証する。
pub(super) fn 片付ける(device: &GPUデバイス, 組一覧: &[(vk::Buffer, vk::DeviceMemory)]) {
    for (バッファ, memory) in 組一覧 {
        unsafe { device.destroy_buffer(*バッファ, None) };
        device.メモリを解放する(*memory);
    }
}

/// 確保した本数が4本と一致することは呼び出し元の繰り返しが保証している。一致しない状態は呼び出し元の誤りである。
pub(super) fn 配列にする(確保済み: Vec<(vk::Buffer, vk::DeviceMemory)>) -> [(vk::Buffer, vk::DeviceMemory); 4] {
    let 本数 = 確保済み.len();
    確保済み
        .try_into()
        .unwrap_or_else(|_| panic!("合成入力のバッファを4本確保するはずが{本数}本だった(実装のバグ)"))
}
