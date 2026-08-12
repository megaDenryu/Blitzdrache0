//! GPU粒子トイ(判断29)一式: 粒子ストレージバッファ・専用ディスクリプタ・
//! コンピュート更新パイプライン・粒子描画パイプラインをまとめて生成・破棄する。
//! `--particles`指定時のみレンダラーがこれを保持する(既定は無効)。

mod buffer;
mod compute_pipeline;
mod descriptor;
mod draw_pipeline;
mod inputs;

use buffer::粒子バッファ;
use compute_pipeline::粒子コンピュートパイプライン;
use descriptor::粒子ディスクリプタ一式;
use draw_pipeline::粒子描画パイプライン;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::particle_material::粒子素材;
use crate::particle_shader_set::粒子シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(crate) struct 粒子リソース一式 {
    バッファ: 粒子バッファ,
    ディスクリプタ: 粒子ディスクリプタ一式,
    コンピュートパイプライン: 粒子コンピュートパイプライン,
    描画パイプライン: 粒子描画パイプライン,
    更新スレッド数: u32,
    描画要素数: u32,
}

impl 粒子リソース一式 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &転送実行環境,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        シェーダー定数: &フレームシェーダー定数一式,
        シェーダー: &粒子シェーダー一式,
        素材: &粒子素材,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let バッファ = 粒子バッファ::生成する(確保係, 転送環境, 素材)?;
        let ディスクリプタ = match 粒子ディスクリプタ一式::生成する(device, バッファ.バッファのハンドル(), シェーダー定数)
        {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let コンピュートパイプライン =
            match 粒子コンピュートパイプライン::生成する(確保係, ディスクリプタ.layout, シェーダー.コンピュートコード())
            {
                Ok(パイプライン) => パイプライン,
                Err(誤り) => {
                    ディスクリプタ.破棄する(device);
                    バッファ.破棄する(device);
                    return Err(誤り);
                }
            };
        let 描画パイプライン = match 粒子描画パイプライン::生成する(
            確保係,
            カラー形式,
            深度形式,
            ディスクリプタ.layout,
            シェーダー.頂点コード(),
            シェーダー.画素段コード(),
        ) {
            Ok(パイプライン) => パイプライン,
            Err(誤り) => {
                コンピュートパイプライン.破棄する(device);
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };

        Ok(Self {
            バッファ,
            ディスクリプタ,
            コンピュートパイプライン,
            描画パイプライン,
            更新スレッド数: 素材.更新スレッド数,
            描画要素数: 素材.描画要素数,
        })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.描画パイプライン.破棄する(device);
        self.コンピュートパイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
