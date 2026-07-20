//! コマンドプール/バッファ・フレーム同期・提示同期・パイプライン(シーン+シャドウ)の
//! 組み立て。`generate_resources`の行数分割のためだけに切り出した内部ヘルパー。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan;
use crate::vulkan::depth::深度形式;
use crate::vulkan::sync::フレームインフライト数;

pub(super) struct コマンド同期資源 {
    pub(super) command_pool: vk::CommandPool,
    pub(super) command_buffer一覧: [vk::CommandBuffer; フレームインフライト数],
    pub(super) フレーム同期: vulkan::sync::フレーム同期,
    pub(super) 提示同期: vulkan::sync::提示同期,
    pub(super) pipeline: vulkan::pipeline::パイプライン,
    pub(super) シャドウパイプライン: vulkan::pipeline::シャドウパイプライン,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    device: &ash::Device,
    queue_family_index: u32,
    swapchain: &vulkan::swapchain::スワップチェーン,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー一式,
    シャドウシェーダー: &シェーダー一式,
) -> Result<コマンド同期資源, レンダラーエラー> {
    let (command_pool, command_buffer一覧) = vulkan::commands::生成する(device, queue_family_index)?;
    let フレーム同期 = vulkan::sync::フレーム同期::生成する(device)?;
    let 提示同期 = vulkan::sync::提示同期::生成する(device, swapchain.画像数())?;
    let pipeline = vulkan::pipeline::パイプライン::生成する(
        device,
        swapchain.画像形式,
        深度形式,
        ディスクリプタlayout,
        シェーダー,
    )?;
    let シャドウパイプライン =
        vulkan::pipeline::シャドウパイプライン::生成する(device, ディスクリプタlayout, シャドウシェーダー)?;

    Ok(コマンド同期資源 {
        command_pool,
        command_buffer一覧,
        フレーム同期,
        提示同期,
        pipeline,
        シャドウパイプライン,
    })
}
