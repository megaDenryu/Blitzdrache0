//! 布一式の生成手順。途中失敗時は生成済みの資源を逆順で片付ける。

use ash::vk;

use super::{buffers, descriptor, params, pipelines, 布一式};
use crate::cloth_material::布素材;
use crate::cloth_shader_set::布シェーダー一式;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::pipeline::パイプライン;
use crate::vulkan::transfer::転送実行環境;

#[allow(clippy::too_many_arguments)]
pub(crate) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    転送環境: &転送実行環境,
    シーンカラー形式: vk::Format,
    セットレイアウト: &crate::vulkan::descriptor::シーンセットレイアウト一式,
    素材: &布素材,
    シェーダー: &布シェーダー一式,
    スキン済み頂点buffer: Option<vk::Buffer>,
) -> Result<布一式, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let バッファ = buffers::生成する(確保係, 転送環境, 素材)?;
    let ディスクリプタ = match descriptor::生成する(device, &バッファ, スキン済み頂点buffer) {
        Ok(一式) => 一式,
        Err(誤り) => {
            バッファ.破棄する(device);
            return Err(誤り);
        }
    };
    let パイプライン群 = match pipelines::生成する(確保係, ディスクリプタ.layout, シェーダー) {
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
        一辺粒子数: 素材.一辺粒子数,
        アタッチ件数: u32::try_from(素材.アタッチ対応一覧.len()).unwrap_or_else(|_| panic!("アタッチ件数がu32に収まらない")),
        既定逆質量: 既定逆質量を求める(&素材.粒子バイト列),
    };
    let インデックス数 = u32::try_from(素材.インデックス一覧.len()).unwrap_or_else(|_| panic!("インデックス数がu32に収まらない"));
    Ok(布一式 {
        バッファ,
        ディスクリプタ,
        パイプライン群,
        描画パイプライン,
        固定部,
        インデックス数,
    })
}

/// 前提: 粒子バイト列は検証済み(32バイト/粒子、各粒子の開始位置12にf32逆質量)。
/// 「離す」介入の復元値のため、可動粒子の逆質量が要る。先頭(上端)は吊るし布で固定(逆質量0)に
/// なりうるため、末尾(下端)の粒子から読む(判断56)。
fn 既定逆質量を求める(粒子バイト列: &[u8]) -> f32 {
    let mut 成分 = [0u8; 4];
    let 末尾開始位置 = 粒子バイト列.len() - 32 + 12;
    成分.copy_from_slice(&粒子バイト列[末尾開始位置..末尾開始位置 + 4]);
    f32::from_le_bytes(成分)
}
