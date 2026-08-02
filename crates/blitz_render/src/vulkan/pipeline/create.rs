//! パイプライン生成のモジュールライフサイクル管理: シェーダーモジュールの生成→
//! 固定機能組み立て→破棄を担う。固定機能ステートの詳細は`graphics_pipeline`に委ねる。

use ash::vk;

use super::{graphics_pipeline, パイプライン};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

pub(super) fn 生成する(
    device: &ash::Device,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
    シェーダー: &シェーダー一式,
    属性選択: graphics_pipeline::頂点属性選択,
    プッシュ定数範囲: vk::PushConstantRange,
) -> Result<パイプライン, レンダラーエラー> {
    let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
    let 画素段モジュール = match shader_module::生成する(device, シェーダー.画素段コード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };

    let 結果 = graphics_pipeline::組み立てる(
        device,
        カラー形式,
        深度形式,
        ディスクリプタlayout一覧,
        頂点モジュール,
        画素段モジュール,
        属性選択,
        プッシュ定数範囲,
    );

    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(画素段モジュール, None);
    }
    結果
}
