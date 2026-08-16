//! 部品で組んだ家の並びの世界が焼くものの宣言。担当するのは、カタログへ載せる部品7件と、何軒建てるかである。
//! 宣言の型と、宣言を指定へ写す手順は`part_row_declaration`が持ち、組み立て規則そのものは同じ場所の`house_rule`が持つ。
//!
//! 煙突は差込面の種別と寸法が是正されて再収蔵されたため、7部品目として宣言に入っている
//! (参照: `_doc/設計/部品カタログと接合点.md`「実データが暴いた契約の穴」穴2)。
//!
//! 10軒と100軒の2つの規模を持つのは、段4の実証が「発行数が件数に依らない」ことだからである。
//! 1つの規模だけでは、発行数がたまたまその件数と一致しているのか件数に依らないのかを見分けられない。

use super::super::archetype_identity::原型の識別;
use super::super::catalog::{アセット定義, ソース種別};
use super::assembly_rule_choice::部品の組み立て規則の種類;
use super::part_row_declaration::{部品で組んだ並びの宣言, 部品のアセット定義一覧を作る};

/// 家を組み立てる部品の一覧。原型はすべて外部のアセットリポジトリの`parts/`から引く。
/// 部品IDはglTFのファイル名から決まるため、この表はファイル名と安定IDの対応だけを持つ。
const 家の部品一覧: &[原型の識別] = &[
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

/// この世界がチャンク以外に焼くもの。
pub(super) fn 一覧() -> Vec<アセット定義> {
    部品のアセット定義一覧を作る(家の部品一覧)
}

pub(super) fn チャンクのソース種別(規模: 家の並びの規模) -> ソース種別 {
    ソース種別::部品で組んだ並び(部品で組んだ並びの宣言::生成する(
        家の部品一覧,
        規模.件数(),
        部品の組み立て規則の種類::酒場宿屋,
    ))
}
