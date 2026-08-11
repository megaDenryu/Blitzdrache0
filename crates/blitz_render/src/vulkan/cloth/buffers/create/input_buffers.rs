//! 描画インデックスとスキン頂点へのアタッチ対応をGPUバッファへ転送する。

use ash::vk;

use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ, 巻き戻せる確保の台帳};
use crate::vulkan::geometry::upload;
use crate::vulkan::transfer::転送実行環境;

pub(super) fn 生成する(
    台帳: &mut 巻き戻せる確保の台帳<'_, '_>,
    確保係: &GPU資源の確保係<'_>,
    転送環境: &転送実行環境,
    素材: &布素材,
    ストレージ: vk::BufferUsageFlags,
) -> Result<(専用メモリ付きバッファ, 専用メモリ付きバッファ), レンダラーエラー> {
    let mut インデックスバイト列 = Vec::with_capacity(素材.インデックス一覧.len() * 4);
    for 添字 in &素材.インデックス一覧 {
        インデックスバイト列.extend_from_slice(&添字.to_le_bytes());
    }
    let インデックス = 台帳.積む(upload::ステージング経由でアップロードする(
        確保係,
        転送環境,
        &インデックスバイト列,
        vk::BufferUsageFlags::INDEX_BUFFER,
    ))?;

    let mut アタッチバイト列 = Vec::with_capacity((素材.アタッチ対応一覧.len() * 8).max(8));
    for 対応 in &素材.アタッチ対応一覧 {
        アタッチバイト列.extend_from_slice(&対応[0].to_le_bytes());
        アタッチバイト列.extend_from_slice(&対応[1].to_le_bytes());
    }
    // アタッチ0件でも0バイト確保はVulkanの契約違反のため、読まれない8バイトのダミーを置く。
    if アタッチバイト列.is_empty() {
        アタッチバイト列.extend_from_slice(&[0u8; 8]);
    }
    let アタッチ = 台帳.積む(upload::ステージング経由でアップロードする(
        確保係,
        転送環境,
        &アタッチバイト列,
        ストレージ,
    ))?;
    Ok((インデックス, アタッチ))
}
