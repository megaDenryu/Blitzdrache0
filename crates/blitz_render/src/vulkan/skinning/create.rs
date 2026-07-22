//! スキニングのバッファ、ディスクリプタ、パイプラインを順に生成する。

use ash::vk;

use super::{buffers, descriptor, pipeline, スキニング一式};
use crate::compute_shader::コンピュートシェーダー;
use crate::error::レンダラーエラー;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

impl スキニング一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        頂点一覧: &[頂点],
        素材: &スキンメッシュ素材,
        シェーダー: &コンピュートシェーダー,
    ) -> Result<Self, レンダラーエラー> {
        if 素材.属性一覧().len() != 頂点一覧.len() {
            return Err(レンダラーエラー::スキン属性数不一致 {
                属性数: 素材.属性一覧().len(),
                頂点数: 頂点一覧.len(),
            });
        }
        let 頂点数 = u32::try_from(頂点一覧.len()).unwrap_or_else(|_| panic!("頂点数がu32に収まらない: {}", 頂点一覧.len()));

        let バッファ = buffers::生成する(device, メモリプロパティ, 転送環境, 頂点一覧, 素材)?;
        let ディスクリプタ = match descriptor::生成する(device, &バッファ) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let パイプライン = match pipeline::生成する(device, ディスクリプタ.layout, シェーダー.コード()) {
            Ok(パイプライン) => パイプライン,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            バッファ,
            ディスクリプタ,
            パイプライン,
            頂点数,
            ジョイント数: 素材.ジョイント数(),
        })
    }
}
