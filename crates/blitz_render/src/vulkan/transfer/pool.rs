//! セットアップ時専用の転送コマンドプール生成。フレーム描画用のコマンドプール
//! (`vulkan::commands`)とは独立させ、頂点/インデックス/テクスチャの初回アップロードと
//! ホットリロード時の再アップロードの両方から使い回す。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 生成する(
    device: &ash::Device,
    キューファミリ添字: u32,
) -> Result<vk::CommandPool, レンダラーエラー> {
    let create_info = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::TRANSIENT)
        .queue_family_index(キューファミリ添字);
    // 安全性: deviceは生成済みで有効。キューファミリ添字は選定済みの正当な値。
    Ok(unsafe { device.create_command_pool(&create_info, None)? })
}
