//! 見本の集落の世界が焼くものの宣言。担当するのは、地面のチャンクへ同居させる小物の種と個体数と並べ方を1箇所へ集めることだけである。
//! 並べ方の意味は`blitz_asset_compiler`の`配置様式`が持ち、宣言1件の組み立てと指定への写しは`prop_group_declaration`が持つ。
//!
//! 数の配分は絵から決めた。囲いを作る柵と杭と石垣は輪1周を埋める本数を、荷と器は使う場所へ寄せた塊に見える数を、
//! 岩と切り株は地面の単調さが消える密度を選んである。合計は1035体である。
//! 参照: `_doc/設計/Blenderアセット運用.md`「段3: 小物の量産と見本の集落」

use super::super::catalog::アセット定義;
use super::super::source_kind::原型と置き方の宣言;
use super::definition_kind::外部ソース専用定義;
use super::prop_group_declaration::{小物, 散らす, 輪, 集まり};

/// 集落を作る小物の一覧。原型はすべて外部のアセットリポジトリの`props/`から引く。
/// 囲いの3種は半径の違う3重の輪になり、内から石垣・柵・杭の順に外へ広がる。荷と器の4種は輪の内側の4箇所へ寄せてある。
pub(super) const 集落の小物一覧: &[原型と置き方の宣言] = &[
    小物("village_stone_wall", "props/stone_wall.glb", 輪(40, 15.0, 0.5)),
    小物("village_fence_section", "props/fence_section.glb", 輪(96, 26.0, 0.6)),
    小物("village_wooden_stake", "props/wooden_stake.glb", 輪(120, 30.5, 0.9)),
    小物("village_wooden_crate", "props/wooden_crate.glb", 集まり(60, 58.0, 44.0, 5.0)),
    小物("village_barrel", "props/barrel.glb", 集まり(55, 42.0, 56.0, 5.0)),
    小物("village_clay_pot", "props/clay_pot.glb", 集まり(48, 44.0, 42.5, 4.0)),
    小物("village_firewood_pile", "props/firewood_pile.glb", 集まり(36, 56.5, 57.5, 5.0)),
    小物("village_tree_stump", "props/tree_stump.glb", 散らす(90, 88.0)),
    小物("village_boulder", "props/boulder.glb", 散らす(70, 84.0)),
    小物("village_rock", "props/rock.glb", 散らす(420, 88.0)),
];

/// この世界がチャンク以外に焼くもの。小物の原型は群が素材として読むだけであり、実行時形式は作らない。
pub(super) fn 一覧() -> Vec<アセット定義> {
    集落の小物一覧
        .iter()
        .map(|宣言| 外部ソース専用定義(宣言.原型の安定id, 宣言.ソース相対パス))
        .collect()
}
