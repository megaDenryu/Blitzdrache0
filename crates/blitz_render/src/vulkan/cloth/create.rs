//! 布一式の生成手順。途中失敗時は生成済みの資源を逆順で片付ける。

use ash::vk;

use super::{buffers, descriptor, params, pipelines, 布一式};
use crate::cloth_material::布素材;
use crate::cloth_shader_set::布シェーダー一式;
use crate::error::レンダラーエラー;
use crate::vulkan::pipeline::パイプライン;
use crate::vulkan::transfer::ステージング経由の転送係;

impl 布一式 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        転送係: ステージング経由の転送係<'_>,
        シーンカラー形式: vk::Format,
        セットレイアウト: &crate::vulkan::descriptor::シーンセットレイアウト一式,
        素材: &布素材,
        シェーダー: &布シェーダー一式,
        スキン済み頂点buffer: Option<vk::Buffer>,
    ) -> Result<Self, レンダラーエラー> {
        let device = 転送係.論理デバイス();
        let 確保係 = 転送係.確保係();
        let バッファ = buffers::布バッファ::生成する(転送係, 素材)?;
        let ディスクリプタ = match descriptor::布ディスクリプタを生成する(device, &バッファ, スキン済み頂点buffer) {
            Ok(一式) => 一式,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let パイプライン群 = match pipelines::布パイプライン群を生成する(確保係, ディスクリプタ.レイアウトのハンドル(), シェーダー)
        {
            Ok(群) => 群,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        // 布描画はシーンと同構成のパイプライン(cull無効・同じ48バイト頂点レイアウト・同じディスクリプタ)。
        let 描画パイプライン = match パイプライン::布用に生成する(
            確保係,
            シーンカラー形式,
            crate::vulkan::depth::深度形式,
            &セットレイアウト.布描画の並び(),
            シェーダー.描画.束縛レイアウトで選ぶ(セットレイアウト.照明束縛()),
        ) {
            Ok(一式) => 一式,
            Err(誤り) => {
                パイプライン群.破棄する(device);
                ディスクリプタ.破棄する(device);
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let 固定部 = params::固定部 {
            定数: 素材.定数,
            粒子数: 素材.粒子数,
            拘束の数: 素材.拘束の数,
            目標拘束の数: 素材.目標拘束の数,
            曲げ拘束の数: 素材.曲げ拘束の数,
            色の区間一覧: 素材.色の区間一覧.clone(),
            曲げの色の区間一覧: 素材.曲げの色の区間一覧.clone(),
            一辺粒子数: 素材.一辺粒子数,
            目標の更新対応の件数: u32::try_from(素材.目標の更新対応一覧.len())
                .unwrap_or_else(|_| panic!("目標の更新対応の件数がu32に収まらない")),
        };
        let インデックス数 = u32::try_from(素材.インデックス一覧.len()).unwrap_or_else(|_| panic!("インデックス数がu32に収まらない"));
        Ok(Self {
            バッファ,
            ディスクリプタ,
            パイプライン群,
            描画パイプライン,
            固定部,
            インデックス数,
        })
    }
}
