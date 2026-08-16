//! glTFの原型と決定的な配置生成から、インスタンス群を1つ持つチャンクの実行時シーンを焼く。
//! 走査順・混合関数の種・格子の刻みをすべて固定しているため、同じ原型と同じ個体数からは常に同じバイト列を得る。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「インスタンス群の表現」

mod bounds;
mod cull_placement;
mod cull_scene;
mod floor_scene;
pub(crate) mod group_object;
mod lod_placement;
mod lod_scene;
mod placement;
mod shadow_range_placement;
mod shadow_range_scene;
mod single_scene;
mod terrain_group;
mod terrain_placement;

pub use terrain_group::同居植生の指定;
#[cfg(test)]
mod vegetation_tests;

use blitz_engine::{アセットID, シーンデータ, チャンク座標};

use crate::assembled_scene::組み立てたシーン;
use crate::compile::コンパイル済みシーン;
use crate::error::アセットコンパイルエラー;
use crate::scene_compiler::ソースアセットのコンパイル係;

/// 植生チャンクの描画対象番号。1つのチャンクが1つの群になるため常にこの値である。
const 植生描画対象番号: u64 = 0;

impl ソースアセットのコンパイル係<'_> {
    pub fn 植生チャンクをコンパイルする(
        &self,
        id: &アセットID,
        所有チャンク: チャンク座標,
        個体数: usize,
    ) -> Result<コンパイル済みシーン, アセットコンパイルエラー> {
        let 組み立て = self.植生チャンクのシーンを組み立てる(id, 所有チャンク, 個体数)?;
        コンパイル済みシーン::組み立てたシーンから焼く(組み立て)
    }

    /// 原型のメッシュLOD列とマテリアルはソースglTFが並べた描画対象から取る。群の全個体が共有するマテリアルは
    /// 外側の描画対象データが持つため、原型はメッシュ列だけを持つ。
    fn 植生チャンクのシーンを組み立てる(
        &self,
        id: &アセットID,
        所有チャンク: チャンク座標,
        個体数: usize,
    ) -> Result<組み立てたシーン, アセットコンパイルエラー> {
        let 原型ソース = self.原型ソースを読み込む(id)?;
        let 配置一覧 = placement::配置列を作る(個体数)?;
        let 群 = group_object::群の描画対象を作る(植生描画対象番号, 所有チャンク, 原型ソース.段一覧, 原型ソース.材質集合, 配置一覧)?;
        let シーン = シーンデータ::生成する(群.描画対象, Vec::new(), 原型ソース.参照ファイル一覧, None, Vec::new());
        Ok(組み立てたシーン::置いた個体の数を添えて生成する(
            シーン,
            群.置いた個体の数,
        ))
    }
}
