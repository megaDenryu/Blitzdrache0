//! シーン描画とシャドウ描画のグラフィクスパイプラインの組み立て。どちらもディスクリプタセットレイアウトとシェーダーを材料に取り、描画先の色形式に合わせて作る。
//! シーン用だけが色アタッチメントへ書くため描画先形式を要り、シャドウ用は深度だけへ書くため要らない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::depth::深度形式;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct パイプライン資源 {
    pub(super) pipeline: vulkan::pipeline::パイプライン,
    pub(super) シャドウパイプライン: vulkan::pipeline::シャドウパイプライン,
}

pub(super) fn 組み立てる(
    device: &GPUデバイス,
    シーンカラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー束,
) -> Result<パイプライン資源, レンダラーエラー> {
    let pipeline = vulkan::pipeline::パイプライン::生成する(device, シーンカラー形式, 深度形式, ディスクリプタlayout, &シェーダー.シーン)?;
    let シャドウパイプライン = vulkan::pipeline::シャドウパイプライン::生成する(device, ディスクリプタlayout, &シェーダー.シャドウ)?;

    Ok(パイプライン資源 {
        pipeline,
        シャドウパイプライン,
    })
}
