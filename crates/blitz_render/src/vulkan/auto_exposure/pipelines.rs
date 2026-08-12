//! 自動露出の2本のコンピュートパイプラインとそのレイアウトの束。担当するのは、2本を同じセットレイアウトの上に順に作り、
//! 途中で失敗したらそこまでのハンドルを逆順に破棄することである。どちらのエントリがどれだけの即時定数を持つかは`layout`が持つ。

mod layout;

use ash::vk;

use super::descriptor::自動露出のディスクリプタ;
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 自動露出のパイプライン一式 {
    pub(crate) 集計: vk::Pipeline,
    pub(crate) 集計レイアウト: vk::PipelineLayout,
    pub(crate) 導出: vk::Pipeline,
    pub(crate) 導出レイアウト: vk::PipelineLayout,
}

impl 自動露出のパイプライン一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタ: &自動露出のディスクリプタ,
        集計シェーダー: &コンピュートシェーダー,
        導出シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 集計レイアウト = layout::集計のレイアウトを作る(device, ディスクリプタ.レイアウト)?;
        match 残りを作る(確保係, ディスクリプタ, 集計シェーダー, 導出シェーダー, 集計レイアウト) {
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
    確保係: &GPU資源の確保係<'_>,
    ディスクリプタ: &自動露出のディスクリプタ,
    集計シェーダー: &コンピュートシェーダー,
    導出シェーダー: &コンピュートシェーダー,
    集計レイアウト: vk::PipelineLayout,
) -> Result<(vk::Pipeline, vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 集計 = 確保係.コンピュートパイプラインを生成する(集計レイアウト, 集計シェーダー.コード(), c"computeMain")?;
    let 導出レイアウト = match layout::導出のレイアウトを作る(device, ディスクリプタ.レイアウト) {
        Ok(レイアウト) => レイアウト,
        Err(誤り) => {
            // 安全性: 集計パイプラインはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_pipeline(集計, None) };
            return Err(誤り);
        }
    };
    match 確保係.コンピュートパイプラインを生成する(導出レイアウト, 導出シェーダー.コード(), c"computeMain") {
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
