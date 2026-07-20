//! GPU粒子トイ(判断29)一式: 粒子ストレージバッファ・専用ディスクリプタ・
//! コンピュート更新パイプライン・粒子描画パイプラインをまとめて生成・破棄する。
//! `--particles`指定時のみレンダラーがこれを保持する(既定は無効)。

mod buffer;
mod compute_pipeline;
mod descriptor;
mod draw_pipeline;
mod initial_data;

pub(crate) use buffer::粒子バッファ;
pub(crate) use compute_pipeline::粒子コンピュートパイプライン;
pub(crate) use descriptor::粒子ディスクリプタ一式;
pub(crate) use draw_pipeline::粒子描画パイプライン;
pub(crate) use initial_data::粒子数;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::particle_shader_set::粒子シェーダー一式;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(crate) struct 粒子リソース一式 {
    pub(crate) バッファ: 粒子バッファ,
    pub(crate) ディスクリプタ: 粒子ディスクリプタ一式,
    pub(crate) コンピュートパイプライン: 粒子コンピュートパイプライン,
    pub(crate) 描画パイプライン: 粒子描画パイプライン,
}

impl 粒子リソース一式 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ユニフォーム: &フレームユニフォーム一式,
        シェーダー: &粒子シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let バッファ = 粒子バッファ::生成する(device, メモリプロパティ, 転送環境)?;
        let ディスクリプタ = match 粒子ディスクリプタ一式::生成する(device, バッファ.buffer, ユニフォーム) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let コンピュートパイプライン =
            match 粒子コンピュートパイプライン::生成する(device, ディスクリプタ.layout, シェーダー.コンピュートコード()) {
                Ok(パイプライン) => パイプライン,
                Err(誤り) => {
                    ディスクリプタ.破棄する(device);
                    バッファ.破棄する(device);
                    return Err(誤り);
                }
            };
        let 描画パイプライン = match 粒子描画パイプライン::生成する(
            device,
            カラー形式,
            深度形式,
            ディスクリプタ.layout,
            シェーダー.頂点コード(),
            シェーダー.フラグメントコード(),
        ) {
            Ok(パイプライン) => パイプライン,
            Err(誤り) => {
                コンピュートパイプライン.破棄する(device);
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };

        Ok(Self { バッファ, ディスクリプタ, コンピュートパイプライン, 描画パイプライン })
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        self.描画パイプライン.破棄する(device);
        self.コンピュートパイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
