//! 部品で組んだ家の並びの世界が焼くものの宣言。担当するのは、カタログへ載せる部品6件と、何軒建てるかと、
//! 宣言をコンパイラが受け取る指定へ写すことである。組み立て規則そのものは`house_rule`が持つ。
//!
//! 煙突は差込面の種別と寸法が是正されて再収蔵されたため、7部品目として宣言に入っている
//! (参照: `_doc/設計/部品カタログと接合点.md`「実データが暴いた契約の穴」穴2)。
//!
//! 10軒と100軒の2つの規模を持つのは、段4の実証が「発行数が件数に依らない」ことだからである。
//! 1つの規模だけでは、発行数がたまたまその件数と一致しているのか件数に依らないのかを見分けられない。

mod house_rule;

use blitz_asset_compiler::{部品で組んだ並びの件数, 部品で組んだ並びの指定};
use blitz_engine::アセットID;

use super::super::archetype_identity::原型の識別;
use super::super::catalog::{アセット定義, ソース種別};
use super::definition_kind::外部ソース専用定義;

/// 家を組み立てる部品の一覧。原型はすべて外部のアセットリポジトリの`parts/`から引く。
/// 部品IDはglTFのファイル名から決まるため、この表はファイル名と安定IDの対応だけを持つ。
pub(super) const 家の部品一覧: &[原型の識別] = &[
    原型の識別::生成する("part_tavern_f1_base", "parts/Mod_Tavern_F1_Base.glb"),
    原型の識別::生成する("part_tavern_f2_jetty", "parts/Mod_Tavern_F2_Jetty.glb"),
    原型の識別::生成する("part_tavern_f3_jetty", "parts/Mod_Tavern_F3_Jetty.glb"),
    原型の識別::生成する("part_tavern_roof_gable", "parts/Mod_Tavern_Roof_Gable.glb"),
    原型の識別::生成する("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
    原型の識別::生成する("part_tavern_dormer_roof", "parts/Mod_Tavern_Dormer_Roof.glb"),
    原型の識別::生成する("part_tavern_chimney", "parts/Mod_Tavern_Chimney.glb"),
];

/// 家の並びの規模。同じ地面と同じ規則で件数だけが違う2つの世界を、この枝が分ける。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 家の並びの規模 {
    十軒,
    百軒,
}

impl 家の並びの規模 {
    fn 件数(self) -> usize {
        match self {
            Self::十軒 => 10,
            Self::百軒 => 100,
        }
    }
}

/// 家並み1つの宣言。何の部品で何軒建てるかを1つの値で持つ。`ソース種別`を`Copy`のまま保つため、部品を静的な並びで指す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 部品で組んだ並びの宣言 {
    部品一覧: &'static [原型の識別],
    件数: usize,
}

/// この世界がチャンク以外に焼くもの。部品の原型は家並みが素材として読むだけであり、実行時形式は作らない。
pub(super) fn 一覧() -> Vec<アセット定義> {
    家の部品一覧
        .iter()
        .map(|識別| 外部ソース専用定義(識別.安定id(), 識別.ソース相対パス()))
        .collect()
}

pub(super) fn チャンクのソース種別(規模: 家の並びの規模) -> ソース種別 {
    ソース種別::部品で組んだ並び(部品で組んだ並びの宣言 {
        部品一覧: 家の部品一覧,
        件数: 規模.件数(),
    })
}

/// 宣言が持つ安定IDの綴りを検証済みのアセットIDへ写し、規則と件数を添えて指定を組む。
pub(crate) fn 部品で組んだ並びの指定を作る(
    宣言: 部品で組んだ並びの宣言
) -> Result<部品で組んだ並びの指定, String> {
    let 安定id一覧 = 宣言
        .部品一覧
        .iter()
        .map(|識別| アセットID::生成する(識別.安定id()).map_err(|誤り| 誤り.to_string()))
        .collect::<Result<Vec<アセットID>, String>>()?;
    let 件数 = 部品で組んだ並びの件数::生成する(宣言.件数).map_err(|誤り| 誤り.to_string())?;
    部品で組んだ並びの指定::生成する(安定id一覧, house_rule::酒場宿屋の規則()?, 件数, house_rule::家の並びの展開の種())
        .map_err(|誤り| 誤り.to_string())
}
