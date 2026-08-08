//! 時間再構成の全画面三角形パイプラインとそのレイアウトの所有。担うのはシェーダーモジュールの生存期間の管理と、
//! 生成に失敗したときの後始末である。固定機能ステートの構築は`fixed_function`が持つ。
//!
//! 全画面パスの共通の組み立て(`fullscreen_pipeline`)を使わないのは、この1本だけがカラー添付を2枚持つためである。
//! 共通の組み立ては1枚を前提に固定機能を組んでおり、枚数を引数へ足すと1枚のパス4本すべてがその引数を運ぶことになる。

mod fixed_function;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

pub(super) struct 時間再構成のパイプライン {
    pub(super) pipeline: vk::Pipeline,
    pub(super) レイアウト: vk::PipelineLayout,
}

impl 時間再構成のパイプライン {
    pub(super) fn 生成する(
        device: &ash::Device,
        セットレイアウト: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
        let 画素段モジュール = match shader_module::生成する(device, シェーダー.画素段コード()) {
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
        結果.map(|(pipeline, レイアウト)| Self { pipeline, レイアウト })
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.レイアウト, None);
        }
    }
}
