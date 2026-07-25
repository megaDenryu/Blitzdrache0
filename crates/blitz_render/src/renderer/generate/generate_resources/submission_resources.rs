//! フレーム送信に要る資源の組み立て: フレーム進行(コマンドバッファとフレームごとの同期物)と、スワップチェーン画像ごとの提示同期。
//! この段が必要とするのはキューファミリ添字とスワップチェーンの画像数だけで、ディスクリプタセットレイアウトもシェーダーも要らない。

use crate::error::レンダラーエラー;
use crate::renderer::frame_progress::フレーム進行;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct フレーム送信資源 {
    pub(super) フレーム進行: フレーム進行,
    pub(super) 提示同期: vulkan::sync::提示同期,
}

pub(super) fn 組み立てる(
    device: &GPUデバイス,
    キューファミリ添字: u32,
    swapchain: &vulkan::swapchain::スワップチェーン,
) -> Result<フレーム送信資源, レンダラーエラー> {
    let フレーム進行 = フレーム進行::生成する(device, キューファミリ添字)?;
    let 提示同期 = vulkan::sync::提示同期::生成する(device, swapchain.画像数())?;
    Ok(フレーム送信資源 {
        フレーム進行, 提示同期
    })
}
