//! 空中遠近合成パイプライン生成のモジュールライフサイクル管理: シェーダーモジュールの生成→固定機能組み立て→破棄。
//! 固定機能ステートの詳細は`assemble`に委ねる(空パイプラインと同じ分け方)。

use ash::vk;

use super::assemble;
use super::空中遠近合成パイプライン;
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 空中遠近合成パイプラインを生成する(
    確保係: &GPU資源の確保係<'_>,
    カラー形式: vk::Format,
    ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
    シェーダー: &シェーダー一式,
) -> Result<空中遠近合成パイプライン, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 頂点モジュール = 確保係.シェーダーモジュールを生成する(シェーダー.頂点コード())?;
    let 画素段モジュール = match 確保係.シェーダーモジュールを生成する(シェーダー.画素段コード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };

    let 結果 = assemble::組み立てる(device, カラー形式, ディスクリプタlayout一覧, 頂点モジュール, 画素段モジュール);

    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(画素段モジュール, None);
    }
    結果
}
