//! 選別のコンピュートパイプラインとそのレイアウト。担当するのは、どのセットレイアウトを束縛し、
//! 何バイトの即時定数を受けるかを決めることである。
//!
//! 生成の局面をディスクリプタと分けるのは、片方の生成が失敗したときにもう片方を確実に破棄する順を、
//! 呼び出し元の1箇所で読めるようにするためである。

use ash::vk;

use super::view_rows::即時定数のバイト数;
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) struct クラスタ選別のパイプライン {
    pub(super) パイプライン: vk::Pipeline,
    pub(super) レイアウト: vk::PipelineLayout,
}

impl クラスタ選別のパイプライン {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        セットレイアウト: vk::DescriptorSetLayout,
        シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let レイアウト = レイアウトを作る(device, セットレイアウト)?;
        match 確保係.コンピュートパイプラインを生成する(レイアウト, シェーダー.コード(), c"computeMain") {
            Ok(パイプライン) => Ok(Self {
                パイプライン, レイアウト
            }),
            Err(誤り) => {
                // 安全性: レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_pipeline_layout(レイアウト, None) };
                Err(誤り)
            }
        }
    }

    /// 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.パイプライン, None);
            device.destroy_pipeline_layout(self.レイアウト, None);
        }
    }
}

/// GPU境界: 即時定数のバイト数をVulkanが受ける型へ狭める。値は48であり、収まらないことは起こらない。
fn 即時定数として押し込むバイト数() -> u32 {
    u32::try_from(即時定数のバイト数).unwrap_or_else(|_| panic!("即時定数のバイト数がu32に収まらない"))
}

fn レイアウトを作る(
    device: &ash::Device, セットレイアウト: vk::DescriptorSetLayout
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let セット一覧 = [セットレイアウト];
    let 範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(即時定数として押し込むバイト数())];
    let 生成情報 = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&セット一覧)
        .push_constant_ranges(&範囲一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_pipeline_layout(&生成情報, None)? })
}
