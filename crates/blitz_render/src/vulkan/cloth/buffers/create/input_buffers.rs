//! 描画インデックスをGPUバッファへ転送する。

use ash::vk;

use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{専用メモリ付きバッファ, 巻き戻せる確保の台帳};
use crate::vulkan::transfer::ステージング経由の転送係;

pub(super) fn 布の描画インデックスのバッファを生成する(
    台帳: &mut 巻き戻せる確保の台帳<'_, '_>,
    転送係: ステージング経由の転送係<'_>,
    素材: &布素材,
) -> Result<専用メモリ付きバッファ, レンダラーエラー> {
    let mut インデックスバイト列 = Vec::with_capacity(素材.インデックス一覧.len() * 4);
    for 添字 in &素材.インデックス一覧 {
        インデックスバイト列.extend_from_slice(&添字.to_le_bytes());
    }
    台帳.積む(転送係.データからデバイスローカルバッファを確保する(&インデックスバイト列, vk::BufferUsageFlags::INDEX_BUFFER))
}
