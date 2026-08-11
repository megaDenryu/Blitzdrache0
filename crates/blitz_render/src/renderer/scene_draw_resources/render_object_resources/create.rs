//! 描画対象1つぶんのGPU資源を確保する局面。束の読込時に1回だけ呼ばれ、以後この経路は動かない。
//! 途中で失敗したときに確保済みの資源を逆順で解放する規律をここが持つため、毎フレームの読み取り側にはその分岐が現れない。

use super::bounding_sphere::描画対象の境界球;
use super::geometry_list::段別ジオメトリ;
use super::instance_record_storage::個体レコードの置き場;
use super::slot_material_ids::スロット別材質ID;
use super::visible_id_content::可視ID列の内容検査;
use super::visible_id_source::可視ID列の出どころ;
use super::描画対象資源;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::instance_transform::content::個体変換内容;
use crate::vulkan::material_table::大域材質ID;

impl 描画対象資源 {
    /// `材質id一覧`は材質資源表がこの対象の材質スロット素材一覧と同じ並びで発番した大域材質IDである。
    /// `動く個体添字一覧`はこの対象について宣言された動く個体の添字であり、空なら個体レコードは静的な1本になる。
    pub(in crate::renderer::scene_draw_resources) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &vulkan::transfer::転送実行環境,
        素材: &描画対象素材,
        材質id一覧: &[大域材質ID],
        動く個体添字一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 個体数 = u32::try_from(素材.個体変換一覧().len()).map_err(|_| レンダラーエラー::個体数過大(素材.個体変換一覧().len()))?;
        let 段別ジオメトリ = 段別ジオメトリ::生成する(確保係, 転送環境, 素材.段一覧())?;
        let 個体変換内容一覧 = match 個体変換内容::一覧を作る(素材.個体変換一覧()) {
            Ok(値) => 値,
            Err(誤り) => {
                段別ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        let 個体変換 = match 個体レコードの置き場::生成する(確保係, 転送環境, &個体変換内容一覧, 動く個体添字一覧)
        {
            Ok(値) => 値,
            Err(誤り) => {
                段別ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        let 可視id列 = match 可視ID列の出どころ::生成する(確保係, 個体数) {
            Ok(値) => 値,
            Err(誤り) => {
                個体変換.破棄する(device);
                段別ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        let 段別の頂点一覧: Vec<&[crate::vertex::頂点]> = 素材.段一覧().iter().map(|段| 段.頂点一覧()).collect();
        Ok(Self {
            境界球: 描画対象の境界球::段と個体から求める(&段別の頂点一覧, 素材.個体変換一覧(), 動く個体添字一覧),
            大域の基準原点: 素材.大域の基準原点(),
            段別ジオメトリ,
            スロット別材質id: スロット別材質ID::組み立てる(素材.材質スロット素材一覧(), 材質id一覧),
            個体変換,
            可視id列,
            内容検査: 可視ID列の内容検査::生成する(個体数),
        })
    }
}
