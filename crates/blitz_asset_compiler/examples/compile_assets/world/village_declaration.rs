//! 見本の集落の世界が焼くものの宣言。担当するのは、地面のチャンクへ同居させる小物の種と個体数と並べ方を1箇所へ集めることと、
//! その宣言をコンパイラが受け取る指定へ写すことである。並べ方の意味は`blitz_asset_compiler`の`配置様式`が持ち、ここは値を選ぶだけである。
//!
//! 数の配分は絵から決めた。囲いを作る柵と杭と石垣は輪1周を埋める本数を、荷と器は使う場所へ寄せた塊に見える数を、
//! 岩と切り株は地面の単調さが消える密度を選んである。合計は1035体である。
//! 参照: `_doc/設計/Blenderアセット運用.md`「段3: 小物の量産と見本の集落」

use blitz_asset_compiler::{小物群の指定, 配置様式};
use blitz_engine::アセットID;

use super::super::catalog::アセット定義;
use super::super::source_kind::小物群宣言;
use super::definition_kind::外部ソース専用定義;

/// 集落を作る小物の一覧。原型はすべて外部のアセットリポジトリの`props/`から引く。
/// 囲いの3種は半径の違う3重の輪になり、内から石垣・柵・杭の順に外へ広がる。荷と器の4種は輪の内側の4箇所へ寄せてある。
pub(super) const 集落の小物一覧: &[小物群宣言] = &[
    小物("village_stone_wall", "props/stone_wall.glb", 40, 輪(15.0, 0.5)),
    小物("village_fence_section", "props/fence_section.glb", 96, 輪(26.0, 0.6)),
    小物("village_wooden_stake", "props/wooden_stake.glb", 120, 輪(30.5, 0.9)),
    小物("village_wooden_crate", "props/wooden_crate.glb", 60, 集まり(58.0, 44.0, 5.0)),
    小物("village_barrel", "props/barrel.glb", 55, 集まり(42.0, 56.0, 5.0)),
    小物("village_clay_pot", "props/clay_pot.glb", 48, 集まり(44.0, 42.5, 4.0)),
    小物("village_firewood_pile", "props/firewood_pile.glb", 36, 集まり(56.5, 57.5, 5.0)),
    小物("village_tree_stump", "props/tree_stump.glb", 90, 散らす(88.0)),
    小物("village_boulder", "props/boulder.glb", 70, 散らす(84.0)),
    小物("village_rock", "props/rock.glb", 420, 散らす(88.0)),
];

/// 宣言1件を組み立てる。表の1行を1行に収めるための束ねであり、フィールド名の並びはこの関数が持つ。
const fn 小物(安定id: &'static str, ソース相対パス: &'static str, 個体数: usize, 配置様式: 配置様式) -> 小物群宣言 {
    小物群宣言 {
        原型の安定id: 安定id,
        ソース相対パス,
        個体数,
        配置様式,
    }
}

const fn 輪(半径メートル: f32, ばらつきメートル: f32) -> 配置様式 {
    配置様式::輪に沿う {
        半径メートル,
        ばらつきメートル,
    }
}

const fn 集まり(中心xメートル: f32, 中心zメートル: f32, 半径メートル: f32) -> 配置様式 {
    配置様式::集まり {
        中心xメートル,
        中心zメートル,
        半径メートル,
    }
}

const fn 散らす(一辺メートル: f32) -> 配置様式 {
    配置様式::一様に散らす { 一辺メートル }
}

/// この世界がチャンク以外に焼くもの。小物の原型は群が素材として読むだけであり、実行時形式は作らない。
pub(super) fn 一覧() -> Vec<アセット定義> {
    集落の小物一覧
        .iter()
        .map(|宣言| 外部ソース専用定義(宣言.原型の安定id, 宣言.ソース相対パス))
        .collect()
}

/// 宣言が持つ安定IDの綴りを検証済みのアセットIDへ写す。宣言側が`Copy`のまま綴りを持つのは、`ソース種別`を`Copy`に保つためである。
pub(super) fn 群の指定一覧を作る(群一覧: &[小物群宣言]) -> Result<Vec<小物群の指定>, String> {
    群一覧
        .iter()
        .map(|宣言| {
            Ok(小物群の指定 {
                原型id: アセットID::生成する(宣言.原型の安定id).map_err(|誤り| 誤り.to_string())?,
                個体数: 宣言.個体数,
                配置様式: 宣言.配置様式,
            })
        })
        .collect()
}
