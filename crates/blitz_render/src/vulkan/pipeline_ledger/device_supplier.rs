//! パイプラインキーから実物のVkPipelineを作る供給元。担当するのは、1つの描画族のシェーダーとレイアウトを借りて、
//! その族のキーだけをパイプラインへ変えることである。
//!
//! 族を1つに絞るのは、シェーダーの取り違えを型で防ぐためである。族が2つぶんのシェーダーを抱えていると、
//! シーンのキーへシャドウのシェーダーを渡す組み合わせが呼び出し側の書き間違いだけで成立してしまう。
//! シェーダーを借りて持つのは、シェーダーの差し替えが「新しいシェーダーで作った供給元から新しい表を構築する」形になるためであり、
//! 供給元の寿命は1回の構築の間だけである。

mod family_dispatch;

use ash::vk;

use crate::error::{パイプライン台帳エラー, レンダラーエラー};
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::pipeline::材質描画族のパイプラインの生成係;

use super::key::パイプラインキー;
use super::layouts::材質描画族のレイアウト;
use super::supplier::パイプライン供給元;

/// 材質を読む描画族の区別。1つの供給元は1つの族だけを作る。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 材質描画族 {
    シーン,
    シャドウ,
    深度プリパス,
}

impl 材質描画族 {
    const fn 名前(self) -> &'static str {
        match self {
            Self::シーン => "シーン",
            Self::シャドウ => "シャドウ",
            Self::深度プリパス => "深度プリパス",
        }
    }
}

pub(super) struct デバイスパイプライン供給元<'借用> {
    pub(super) 生成係: 材質描画族のパイプラインの生成係<'借用>,
    確保係: &'借用 GPU資源の確保係<'借用>,
    族: 材質描画族,
    レイアウト: &'借用 材質描画族のレイアウト,
}

impl<'借用> デバイスパイプライン供給元<'借用> {
    pub(super) const fn 生成する(
        確保係: &'借用 GPU資源の確保係<'借用>,
        族: 材質描画族,
        レイアウト: &'借用 材質描画族のレイアウト,
        シェーダー: &'借用 シェーダー一式,
    ) -> Self {
        Self {
            生成係: 材質描画族のパイプラインの生成係::生成する(確保係, シェーダー),
            確保係,
            族,
            レイアウト,
        }
    }

    fn 族の不一致(&self, キー: パイプラインキー) -> レンダラーエラー {
        パイプライン台帳エラー::族とキーの不一致 {
            族: self.族.名前(),
            記述: キー.記述(),
        }
        .into()
    }
}

impl パイプライン供給元 for デバイスパイプライン供給元<'_> {
    type 実体 = vk::Pipeline;

    fn 生成する(&mut self, キー: パイプラインキー) -> Result<vk::Pipeline, レンダラーエラー> {
        family_dispatch::生成する(self, キー)
    }

    fn 破棄する(&mut self, 実体: vk::Pipeline) {
        pipelineを破棄する(self.確保係.論理デバイス(), 実体);
    }
}

/// 台帳が所有するパイプライン1本の破棄。台帳の3つの表と供給元の差し替えがこの1つを共有する。
/// 前提: 呼び出し元がGPUの使用完了を保証してから呼ぶ。
pub(super) fn pipelineを破棄する(device: &ash::Device, pipeline: vk::Pipeline) {
    // 安全性: pipelineは台帳が唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
    unsafe { device.destroy_pipeline(pipeline, None) };
}
