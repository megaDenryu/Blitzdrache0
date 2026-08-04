//! 自動露出の2本のコンピュートパイプラインとそのレイアウト。担当するのは、どちらのエントリがどれだけの即時定数を持つかを決めることである。
//! セットレイアウトは2本で共有し、集計は寸法2つ、導出は狙いと速さ6つを即時定数で受ける。

use ash::vk;

use super::descriptor::自動露出のディスクリプタ;
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::vulkan::compute_pipeline;

/// 押し込む定数のバイト数。`shaders/auto_exposure_histogram.slang`の`HistogramExtent`(32ビット2つ)と`auto_exposure_resolve.slang`の`ResolveSetting`(単精度6つ)に一致させる。
const 集計の即時定数バイト数: u32 = 8;
const 導出の即時定数バイト数: u32 = 24;

pub(crate) struct 自動露出のパイプライン一式 {
    pub(crate) 集計: vk::Pipeline,
    pub(crate) 集計レイアウト: vk::PipelineLayout,
    pub(crate) 導出: vk::Pipeline,
    pub(crate) 導出レイアウト: vk::PipelineLayout,
}

impl 自動露出のパイプライン一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        ディスクリプタ: &自動露出のディスクリプタ,
        集計シェーダー: &コンピュートシェーダー,
        導出シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        let 集計レイアウト = レイアウトを作る(device, ディスクリプタ.レイアウト, 集計の即時定数バイト数)?;
        match 残りを作る(device, ディスクリプタ, 集計シェーダー, 導出シェーダー, 集計レイアウト) {
            Ok((集計, 導出, 導出レイアウト)) => Ok(Self {
                集計,
                集計レイアウト,
                導出,
                導出レイアウト,
            }),
            Err(誤り) => {
                // 安全性: 集計レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_pipeline_layout(集計レイアウト, None) };
                Err(誤り)
            }
        }
    }

    /// 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.導出, None);
            device.destroy_pipeline_layout(self.導出レイアウト, None);
            device.destroy_pipeline(self.集計, None);
            device.destroy_pipeline_layout(self.集計レイアウト, None);
        }
    }
}

fn 残りを作る(
    device: &ash::Device,
    ディスクリプタ: &自動露出のディスクリプタ,
    集計シェーダー: &コンピュートシェーダー,
    導出シェーダー: &コンピュートシェーダー,
    集計レイアウト: vk::PipelineLayout,
) -> Result<(vk::Pipeline, vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    let 集計 = compute_pipeline::生成する(device, 集計レイアウト, 集計シェーダー.コード(), c"computeMain")?;
    let 導出レイアウト = match レイアウトを作る(device, ディスクリプタ.レイアウト, 導出の即時定数バイト数) {
        Ok(レイアウト) => レイアウト,
        Err(誤り) => {
            // 安全性: 集計パイプラインはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_pipeline(集計, None) };
            return Err(誤り);
        }
    };
    match compute_pipeline::生成する(device, 導出レイアウト, 導出シェーダー.コード(), c"computeMain") {
        Ok(導出) => Ok((集計, 導出, 導出レイアウト)),
        Err(誤り) => {
            // 安全性: 導出レイアウトと集計パイプラインはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_pipeline_layout(導出レイアウト, None);
                device.destroy_pipeline(集計, None);
            }
            Err(誤り)
        }
    }
}

fn レイアウトを作る(
    device: &ash::Device,
    セットレイアウト: vk::DescriptorSetLayout,
    即時定数バイト数: u32,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let セット一覧 = [セットレイアウト];
    let 範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(即時定数バイト数)];
    let 生成情報 = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&セット一覧)
        .push_constant_ranges(&範囲一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_pipeline_layout(&生成情報, None)? })
}
