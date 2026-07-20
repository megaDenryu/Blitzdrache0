//! シャドウパイプライン生成のモジュールライフサイクル管理: シェーダーモジュールの
//! 生成→固定機能組み立て→破棄を担う(グラフィックスパイプラインと同じ様式)。

use ash::vk;

use super::assemble;
use super::super::シャドウパイプライン;
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

pub(super) fn 生成する(
    device: &ash::Device,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー一式,
) -> Result<シャドウパイプライン, レンダラーエラー> {
    let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
    let フラグメントモジュール = match shader_module::生成する(device, シェーダー.フラグメントコード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };

    let 結果 = assemble::組み立てる(device, ディスクリプタlayout, 頂点モジュール, フラグメントモジュール);

    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(フラグメントモジュール, None);
    }
    結果
}
