//! 時間再構成の全画面三角形パイプラインとそのレイアウトの所有。担うのはシェーダーモジュールの生存期間の管理と、
//! 生成に失敗したときの後始末である。固定機能ステートの構築は`fixed_function`が持つ。
//!
//! 全画面パスの共通の組み立て(`fullscreen_pipeline`)を使わないのは、この1本だけがカラー添付を2枚持つためである。
//! 共通の組み立ては1枚を前提に固定機能を組んでおり、枚数を引数へ足すと1枚のパス4本すべてがその引数を運ぶことになる。

mod fixed_function;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;

pub(super) struct 時間再構成のパイプライン(全画面パスのパイプライン);

impl 時間再構成のパイプライン {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        セットレイアウト: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
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
        let 結果 = fixed_function::二枚書きを組み立てる(device, セットレイアウト, 頂点モジュール, 画素段モジュール);
        // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
        unsafe {
            device.destroy_shader_module(頂点モジュール, None);
            device.destroy_shader_module(画素段モジュール, None);
        }
        結果.map(Self)
    }

    pub(super) const fn パイプラインのハンドル(&self) -> vk::Pipeline {
        self.0.パイプラインのハンドル()
    }

    pub(super) const fn パイプラインレイアウトのハンドル(&self) -> vk::PipelineLayout {
        self.0.パイプラインレイアウトのハンドル()
    }

    /// 前提: 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.0.破棄する(device);
    }
}
