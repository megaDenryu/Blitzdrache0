//! 描画インデックスとスキン頂点へのアタッチ対応をGPUバッファへ転送する。

use ash::vk;

use super::allocation::積む;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::transfer::転送実行環境;

type バッファとメモリ = (vk::Buffer, vk::DeviceMemory);

pub(super) fn 生成する(
    確保済み: &mut Vec<バッファとメモリ>,
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    素材: &布素材,
    ストレージ: vk::BufferUsageFlags,
) -> Result<(バッファとメモリ, バッファとメモリ), レンダラーエラー> {
    let mut インデックスバイト列 = Vec::with_capacity(素材.インデックス一覧.len() * 4);
    for 添字 in &素材.インデックス一覧 {
        インデックスバイト列.extend_from_slice(&添字.to_le_bytes());
    }
    let インデックス = 積む(
        確保済み,
        device,
        upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &インデックスバイト列,
            vk::BufferUsageFlags::INDEX_BUFFER,
        ),
    )?;

    let mut アタッチバイト列 = Vec::with_capacity((素材.アタッチ対応一覧.len() * 8).max(8));
    for 対応 in &素材.アタッチ対応一覧 {
        アタッチバイト列.extend_from_slice(&対応[0].to_le_bytes());
        アタッチバイト列.extend_from_slice(&対応[1].to_le_bytes());
    }
    // アタッチ0件でも0バイト確保はVulkanの契約違反のため、読まれない8バイトのダミーを置く。
    if アタッチバイト列.is_empty() {
        アタッチバイト列.extend_from_slice(&[0u8; 8]);
    }
    let アタッチ = 積む(
        確保済み,
        device,
        upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &アタッチバイト列, ストレージ),
    )?;
    Ok((インデックス, アタッチ))
}
