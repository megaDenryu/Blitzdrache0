//! 描画段階資源の生成。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに生成したパイプラインをその場で逆順に破棄する。生成の途中経過を外へ出さないため、
//! 部分的に生成された器は呼び出し元から見えない。

use ash::vk;

use super::描画段階資源;
use crate::error::レンダラーエラー;
use crate::frame_composition::{フレーム構成, フレーム段階};
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    シーンカラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー束,
    構成: フレーム構成,
) -> Result<描画段階資源, レンダラーエラー> {
    let シーン = vulkan::pipeline::パイプライン::生成する(
        device,
        シーンカラー形式,
        vulkan::depth::深度形式,
        ディスクリプタlayout,
        &シェーダー.シーン,
    )?;
    let シャドウ = match vulkan::pipeline::シャドウパイプライン::生成する(device, ディスクリプタlayout, &シェーダー.シャドウ)
    {
        Ok(シャドウ) => シャドウ,
        Err(誤り) => {
            シーン.破棄する(device);
            return Err(誤り);
        }
    };
    match 空を生成する(device, シーンカラー形式, ディスクリプタlayout, シェーダー, 構成) {
        Ok(空) => Ok(描画段階資源 {
            シーン, シャドウ, 空
        }),
        Err(誤り) => {
            シャドウ.破棄する(device);
            シーン.破棄する(device);
            Err(誤り)
        }
    }
}

/// 空パイプラインはフレーム構成に空段階があるときだけ作る。無い構成では`None`が「空を描かない」ことを型で表す。
fn 空を生成する(
    device: &GPUデバイス,
    シーンカラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー束,
    構成: フレーム構成,
) -> Result<Option<vulkan::pipeline::空パイプライン>, レンダラーエラー> {
    if !構成.含む(フレーム段階::空) {
        return Ok(None);
    }
    let 空 = vulkan::pipeline::空パイプライン::生成する(
        device,
        シーンカラー形式,
        vulkan::depth::深度形式,
        ディスクリプタlayout,
        &シェーダー.空,
    )?;
    Ok(Some(空))
}
