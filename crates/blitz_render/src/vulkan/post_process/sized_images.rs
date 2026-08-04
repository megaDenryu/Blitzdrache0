//! スワップチェーン寸法に連動する2枚の画像(HDR中間画像と光のにじみピラミッド)の生成工程。
//! 受け取るのは寸法、返すのは2枚の画像である。
//!
//! 独立した工程にするのは、生成と寸法追従という呼び出しタイミングの違う2つがこの同じ2枚を作るためである。
//! 片方だけが作り方を変えると、起動直後とリサイズ後で別の資源になる。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tracked_device::GPUデバイス;

/// ピラミッドの生成に失敗したらHDR中間画像を片付ける。
pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    寸法: vk::Extent2D,
) -> Result<(HDRターゲット, 光のにじみピラミッド), レンダラーエラー> {
    let hdr = HDRターゲット::生成する(device, メモリプロパティ, 寸法)?;
    match 光のにじみピラミッド::生成する(device, メモリプロパティ, 寸法) {
        Ok(ピラミッド) => Ok((hdr, ピラミッド)),
        Err(誤り) => {
            hdr.破棄する(device);
            Err(誤り)
        }
    }
}
