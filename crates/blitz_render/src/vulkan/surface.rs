//! ウィンドウとVulkanの接続点であるサーフェスの生成。

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::error::レンダラーエラー;

/// サーフェスローダーとサーフェスハンドルを生成する。
///
/// 前提: `表示ハンドル` と `ウィンドウハンドル` の指すウィンドウは、戻り値の
/// サーフェスより長生きすることを呼び出し元が保証する。
pub(in crate::vulkan) fn 生成する(
    entry: &ash::Entry,
    instance: &ash::Instance,
    表示ハンドル: RawDisplayHandle,
    ウィンドウハンドル: RawWindowHandle,
) -> Result<(ash::khr::surface::Instance, vk::SurfaceKHR), レンダラーエラー> {
    let loader = ash::khr::surface::Instance::new(entry, instance);

    // 安全性: 表示ハンドル・ウィンドウハンドルの生存は呼び出し元(前提コメント参照)が
    // 保証する。entry・instanceはこの呼び出しの直前に生成済みで有効。
    let surface = unsafe { ash_window::create_surface(entry, instance, 表示ハンドル, ウィンドウハンドル, None)? };

    Ok((loader, surface))
}
