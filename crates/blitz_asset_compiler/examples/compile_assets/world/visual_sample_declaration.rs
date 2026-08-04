//! 目視見本の世界が焼くものの宣言。担当するのは、材質見本の立体と遠景地面と、庭へ同居させる小物の種と個体数と並べ方を1箇所へ集めることだけである。
//! 宣言1件の組み立てと指定への写しは`prop_group_declaration`が持つ。
//!
//! 小物を5種58体に留めるのは、この世界の主役が材質見本の球であり、小物は庭の広さと明るさの手がかりとして置くものだからである。
//! 柵は庭を囲む輪に、荷と岩と切り株は台座を避けた4つの塊に置く。台座はX方向におよそ12メートル・Z方向におよそ3.4メートルを占めるため、
//! 塊の中心はZが庭の中心から4メートル以上離れた位置を選んである。
//! 参照: `_doc/設計/Blenderアセット運用.md`「段3: 小物の量産と見本の集落」

use blitz_asset_compiler::目視見本の指定;
use blitz_engine::アセットID;

use super::super::catalog::アセット定義;
use super::super::source_kind::小物群宣言;
use super::definition_kind::{ソース専用定義, 外部ソース専用定義};
use super::prop_group_declaration::{小物, 群の指定一覧を作る, 輪, 集まり};

/// 材質見本の立体を指す安定ID。庭の地面のチャンクが素材として読むだけであり、実行時形式は作らない。
pub(super) const 材質見本の立体: &str = "terrain_visual_sample_bodies";
const 材質見本の立体のソース: &str = "terrain_visual_world/material_sample_bodies.gltf";

/// チャンクの外へ広がる遠景地面を指す安定ID。こちらも庭の地面のチャンクが素材として読むだけである。
pub(super) const 遠景地面: &str = "terrain_visual_distant_ground";
const 遠景地面のソース: &str = "terrain_visual_world/distant_ground.gltf";

/// 庭を作る小物の一覧。原型はすべて外部のアセットリポジトリの`props/`から引く。
pub(super) const 庭の小物一覧: &[小物群宣言] = &[
    小物("terrain_visual_fence_section", "props/fence_section.glb", 40, 輪(14.0, 0.5)),
    小物("terrain_visual_barrel", "props/barrel.glb", 5, 集まり(43.5, 55.5, 1.8)),
    小物("terrain_visual_wooden_crate", "props/wooden_crate.glb", 5, 集まり(56.5, 55.0, 2.0)),
    小物("terrain_visual_boulder", "props/boulder.glb", 4, 集まり(42.5, 45.0, 2.5)),
    小物("terrain_visual_tree_stump", "props/tree_stump.glb", 4, 集まり(57.5, 44.5, 2.2)),
];

/// この世界がチャンク以外に焼くもの。材質見本の立体も遠景地面も小物の原型も、庭の地面のチャンクが素材として読むだけである。
pub(super) fn 一覧() -> Vec<アセット定義> {
    let mut 定義一覧 = vec![
        ソース専用定義(材質見本の立体, 材質見本の立体のソース),
        ソース専用定義(遠景地面, 遠景地面のソース),
    ];
    定義一覧.extend(庭の小物一覧.iter().map(|宣言| 外部ソース専用定義(宣言.原型の安定id, 宣言.ソース相対パス)));
    定義一覧
}

/// 宣言をコンパイラが受け取る指定へ写す。材質見本の立体の安定IDと小物群の宣言を1つの指定へまとめる。
pub(crate) fn 目視見本の指定を作る(
    材質見本の立体の安定id: &str,
    遠景地面の安定id: &str,
    群一覧: &[小物群宣言],
) -> Result<目視見本の指定, String> {
    Ok(目視見本の指定 {
        材質見本の立体id: アセットID::生成する(材質見本の立体の安定id).map_err(|誤り| 誤り.to_string())?,
        遠景地面id: アセットID::生成する(遠景地面の安定id).map_err(|誤り| 誤り.to_string())?,
        群一覧: 群の指定一覧を作る(群一覧)?,
    })
}
