//! glTFの原型と決定的な配置生成から、インスタンス群を1つ持つチャンクの実行時シーンを焼く。
//! 走査順・混合関数の種・格子の刻みをすべて固定しているため、同じ原型と同じ個体数からは常に同じバイト列を得る。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「インスタンス群の表現」

mod bounds;
mod cull_placement;
mod cull_scene;
pub(crate) mod deterministic_value;
mod floor_scene;
pub(crate) mod group_object;
mod lod_placement;
mod lod_scene;
mod placement;
mod single_scene;
mod terrain_group;
mod terrain_placement;

pub use cull_scene::植生可視判定シーンをコンパイルする;
pub use lod_scene::植生詳細段シーンをコンパイルする;
pub use single_scene::植生単一個体シーンをコンパイルする;
pub use terrain_group::{同居植生の指定, 地形同居の群, 地形同居の群を作る};
#[cfg(test)]
mod vegetation_tests;

use blitz_engine::{アセットID, カタログ, シーンを実行時形式へ格納する, シーンデータ, チャンク座標};

use crate::compile::{コンパイル済みシーン, メタデータを集計する};
use crate::error::アセットコンパイルエラー;
use crate::loader::原型ソースを読み込む;

/// 植生チャンクの描画対象番号。1つのチャンクが1つの群になるため常にこの値である。
const 植生描画対象番号: u64 = 0;

pub fn 植生チャンクをコンパイルする(
    カタログ: &カタログ,
    id: &アセットID,
    所有チャンク: チャンク座標,
    個体数: usize,
) -> Result<コンパイル済みシーン, アセットコンパイルエラー> {
    let シーン = シーンを組み立てる(カタログ, id, 所有チャンク, 個体数)?;
    let メタデータ = メタデータを集計する(&シーン)?;
    let 実行時バイト列 = シーンを実行時形式へ格納する(&シーン)?;
    Ok(コンパイル済みシーン {
        実行時バイト列,
        ソース依存一覧: シーン.参照ファイル一覧,
        メタデータ,
    })
}

/// 原型のメッシュLOD列とマテリアルはソースglTFが並べた描画対象から取る。群の全個体が共有するマテリアルは
/// 外側の描画対象データが持つため、原型はメッシュ列だけを持つ。
fn シーンを組み立てる(
    カタログ: &カタログ,
    id: &アセットID,
    所有チャンク: チャンク座標,
    個体数: usize,
) -> Result<シーンデータ, アセットコンパイルエラー> {
    let 原型ソース = 原型ソースを読み込む(カタログ, id)?;
    let 配置一覧 = placement::配置列を作る(個体数)?;
    let 対象 = group_object::群の描画対象を作る(植生描画対象番号, 所有チャンク, 原型ソース.段一覧, 原型ソース.材質集合, 配置一覧)?;
    Ok(シーンデータ::生成する(
        対象,
        Vec::new(),
        原型ソース.参照ファイル一覧,
        None,
        Vec::new(),
    ))
}
