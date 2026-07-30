//! 空中遠近合成パイプライン: 全画面三角形でシーン色へ霞を混ぜる、深度アタッチメントを持たないグラフィックスパイプライン。
//! set0はシーンのディスクリプタセットレイアウトを再利用してフレームユニフォームを読み、set1で深度とボリュームを参照する。
//! フラグメントの押し込み定数でそのフレームの最遠距離を受ける。
//!
//! 深度をアタッチメントとして束縛しないのは、同じパスで深度を読みながら深度アタッチメントにも指定すると
//! フィードバックループになるためである。画素の限定は深度比較でなくフラグメントの`discard`が担う。

mod assemble;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

/// 押し込み定数のバイト数。`shaders/aerial_composite.slang`の`AerialCompositeCondition`(実数1つ)と一致させる。
const 押し込み定数バイト数: u32 = 4;

pub(crate) struct 空中遠近合成パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 空中遠近合成パイプライン {
    /// `カラー形式`はシーンと同じ色アタッチメント(ポスト処理があればHDR中間画像)の形式である。
    /// `ディスクリプタlayout一覧`はset0から順に並べたレイアウトである。
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, カラー形式, ディスクリプタlayout一覧, シェーダー)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
