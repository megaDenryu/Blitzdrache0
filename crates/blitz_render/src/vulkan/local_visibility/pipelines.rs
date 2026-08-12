//! 局所可視性補正の2本のコンピュートパイプラインとそのレイアウト。
//!
//! セットレイアウトも即時定数の並びも2本で共有する。遮蔽の標本化とぼかしが同じ射影と同じ寸法を読むためであり、
//! 片方だけ別の並びにすると、同じ画面を別の射影で解釈した2枚が重なる。

use ash::vk;

use super::descriptor::局所可視性のディスクリプタ;
use super::setting::即時定数バイト数;
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::local_visibility::局所可視性のシェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 局所可視性のパイプライン一式 {
    pub(crate) 遮蔽の標本化: vk::Pipeline,
    pub(crate) 両側ぼかし: vk::Pipeline,
    /// 2本が共有する1つのレイアウト。セットレイアウトも即時定数の範囲も同じであるため分ける理由が無い。
    pub(crate) レイアウト: vk::PipelineLayout,
}

impl 局所可視性のパイプライン一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタ: &局所可視性のディスクリプタ,
        シェーダー: &局所可視性のシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let レイアウト = レイアウトを作る(device, ディスクリプタ.レイアウトのハンドル())?;
        match 両方を作る(確保係, レイアウト, シェーダー) {
            Ok((遮蔽の標本化, 両側ぼかし)) => Ok(Self {
                遮蔽の標本化,
                両側ぼかし,
                レイアウト,
            }),
            Err(誤り) => {
                // 安全性: レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_pipeline_layout(レイアウト, None) };
                Err(誤り)
            }
        }
    }

    /// 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.両側ぼかし, None);
            device.destroy_pipeline(self.遮蔽の標本化, None);
            device.destroy_pipeline_layout(self.レイアウト, None);
        }
    }
}

fn 両方を作る(
    確保係: &GPU資源の確保係<'_>,
    レイアウト: vk::PipelineLayout,
    シェーダー: &局所可視性のシェーダー一式,
) -> Result<(vk::Pipeline, vk::Pipeline), レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 遮蔽の標本化 = パイプラインを作る(確保係, レイアウト, &シェーダー.遮蔽の標本化)?;
    match パイプラインを作る(確保係, レイアウト, &シェーダー.両側ぼかし) {
        Ok(両側ぼかし) => Ok((遮蔽の標本化, 両側ぼかし)),
        Err(誤り) => {
            // 安全性: 遮蔽の標本化はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_pipeline(遮蔽の標本化, None) };
            Err(誤り)
        }
    }
}

fn パイプラインを作る(
    確保係: &GPU資源の確保係<'_>,
    レイアウト: vk::PipelineLayout,
    シェーダー: &コンピュートシェーダー,
) -> Result<vk::Pipeline, レンダラーエラー> {
    確保係.コンピュートパイプラインを生成する(レイアウト, シェーダー.コード(), c"computeMain")
}

fn レイアウトを作る(
    device: &ash::Device, セットレイアウト: vk::DescriptorSetLayout
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
