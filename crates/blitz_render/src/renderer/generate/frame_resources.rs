//! スワップチェーン生成後に組み立てる残り資源の束。組み立て手順は
//! `generate_resources::組み立てる` が担う。

use ash::vk;

use crate::vulkan;
use crate::vulkan::sync::フレームインフライト数;

pub(super) struct フレーム資源 {
    pub(super) 深度バッファ: vulkan::depth::深度バッファ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) ディスクリプタ: vulkan::descriptor::ディスクリプタ一式,
    pub(super) command_pool: vk::CommandPool,
    pub(super) command_buffer一覧: [vk::CommandBuffer; フレームインフライト数],
    pub(super) フレーム同期: vulkan::sync::フレーム同期,
    pub(super) 提示同期: vulkan::sync::提示同期,
    pub(super) pipeline: vulkan::pipeline::パイプライン,
    pub(super) 粒子: Option<vulkan::particles::粒子リソース一式>,
    pub(super) gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    pub(super) ui一式: vulkan::ui::UIリソース一式,
}
