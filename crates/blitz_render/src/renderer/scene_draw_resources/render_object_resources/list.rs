//! 描画対象素材の一覧から描画対象GPU資源の一覧を生成する。束の中の動く個体の宣言を描画対象ごとの添字の並びへ振り分けるのもここが行う。

use super::描画対象資源;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::render_scene_material::動く個体の宣言;
use crate::vulkan;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::material_table::描画対象別の材質ID;

/// 前提: `材質id一覧`は`描画対象一覧`と同じ並びであり、その内側も各対象の材質スロット素材一覧と同じ並びである
/// (材質資源表への登録がその並びで発番する)。
/// 前提: `動く個体一覧`の指す描画対象添字と個体添字は`描画シーン素材`が範囲内であることを確かめている。
pub(in crate::renderer::scene_draw_resources) fn 描画対象資源一覧を生成する(
    確保係: &GPU資源の確保係<'_>,
    転送環境: &vulkan::transfer::転送実行環境,
    描画対象一覧: &[描画対象素材],
    材質id一覧: &[描画対象別の材質ID],
    動く個体一覧: &[動く個体の宣言],
) -> Result<Vec<描画対象資源>, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    assert_eq!(
        描画対象一覧.len(),
        材質id一覧.len(),
        "描画対象の件数と材質資源表が発番した材質IDの並びの件数が食い違った"
    );
    let mut 一覧 = Vec::with_capacity(描画対象一覧.len());
    for (描画対象添字, (描画対象, 材質id)) in 描画対象一覧.iter().zip(材質id一覧.iter()).enumerate() {
        let 動く個体添字一覧 = 描画対象の動く個体添字を集める(動く個体一覧, 描画対象添字);
        match 描画対象資源::生成する(確保係, 転送環境, 描画対象, 材質id, &動く個体添字一覧) {
            Ok(資源) => 一覧.push(資源),
            Err(誤り) => {
                for 資源 in &一覧 {
                    資源.破棄する(device);
                }
                return Err(誤り);
            }
        }
    }
    Ok(一覧)
}

/// 束ぜんぶの宣言から、その描画対象のぶんだけを添字の並びとして取り出す。宣言が無い対象では空になり、静的な個体変換が選ばれる。
fn 描画対象の動く個体添字を集める(動く個体一覧: &[動く個体の宣言], 描画対象添字: usize) -> Vec<u32> {
    動く個体一覧
        .iter()
        .filter(|宣言| 宣言.描画対象添字 == 描画対象添字)
        .map(|宣言| 宣言.個体添字)
        .collect()
}
