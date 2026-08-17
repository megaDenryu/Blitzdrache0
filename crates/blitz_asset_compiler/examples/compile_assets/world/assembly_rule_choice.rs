//! 部品の組み立て規則の種類と、種類から規則と展開の種の起点を組む引き当て。
//!
//! **並びと散布の両方がこの引き当てを使う。** どちらも「規則を1つ選んで、種で個体ごとに姿を変える」という
//! 同じ形であり、違うのは根をどこへ置くかだけである。宣言が規則そのものでなく種類を持つのは、
//! `ソース種別`を`Copy`のまま保つためである。組み立て規則は候補の並びを`Vec`で持つため`Copy`にできない。
//!
//! 規則そのものは種類ごとのモジュールが持つ。網羅的matchで書くため、種類を1つ足すと規則を書き足すまで通らない。
//! 参照: `_doc/設計/部品カタログと接合点.md`「部品カタログと展開器」

mod frame_rule;
mod house_rule;
mod tree_rule;

use blitz_assembly::{生成の種, 組み立て規則};
use blitz_asset_compiler::散らした種類の名前;

use super::super::archetype_identity::原型の識別;

/// どの規則で1件を組むか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 部品の組み立て規則の種類 {
    酒場宿屋,
    樫の木,
    一間四方の骨格,
}

/// 酒場宿屋を組み立てる部品の一覧。原型はすべて外部のアセットリポジトリの`parts/`から引く。
/// 部品IDはglTFのファイル名から決まるため、この表はファイル名と安定IDの対応だけを持つ。
const 酒場宿屋の部品一覧: &[原型の識別] = &[
    原型の識別::生成する("part_tavern_f1_base", "parts/Mod_Tavern_F1_Base.glb"),
    原型の識別::生成する("part_tavern_f2_jetty", "parts/Mod_Tavern_F2_Jetty.glb"),
    原型の識別::生成する("part_tavern_f3_jetty", "parts/Mod_Tavern_F3_Jetty.glb"),
    原型の識別::生成する("part_tavern_roof_gable", "parts/Mod_Tavern_Roof_Gable.glb"),
    原型の識別::生成する("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
    原型の識別::生成する("part_tavern_dormer_roof", "parts/Mod_Tavern_Dormer_Roof.glb"),
    原型の識別::生成する("part_tavern_chimney", "parts/Mod_Tavern_Chimney.glb"),
];

/// 樫の木を組み立てる部品の一覧。幹の一節を何節積むかは規則の仕事であり、この表は部品の種類を1件ずつ持つだけである。
const 樫の木の部品一覧: &[原型の識別] = &[
    原型の識別::生成する("part_tree_oak_trunk_segment", "parts/Mod_Tree_Oak_Trunk_Segment.glb"),
    原型の識別::生成する("part_tree_oak_branch_large", "parts/Mod_Tree_Oak_Branch_Large.glb"),
    原型の識別::生成する("part_tree_oak_foliage_cluster", "parts/Mod_Tree_Oak_Foliage_Cluster.glb"),
];

/// 一間四方の骨格を組み立てる部品の一覧。骨格1件と壁3種であり、材質スロットは1・2・3・2で総和8である。
/// 壁3種のどれにも必ず入れる面を割り当ててあるため、この4件はどの種でも据わる。
const 一間四方の骨格の部品一覧: &[原型の識別] = &[
    原型の識別::生成する("part_frame_bay_single", "parts/Mod_Frame_Bay_Single.glb"),
    原型の識別::生成する("part_wall_halftimber_solid", "parts/Mod_Wall_HalfTimber_Solid.glb"),
    原型の識別::生成する("part_wall_halftimber_window", "parts/Mod_Wall_HalfTimber_Window.glb"),
    原型の識別::生成する("part_wall_halftimber_doorframe", "parts/Mod_Wall_HalfTimber_DoorFrame.glb"),
];

impl 部品の組み立て規則の種類 {
    /// 報告の内訳へ載せるこの種類の名前。人が読んで何を散らしたのかが分かる語を使う。
    pub(super) fn 名前(self) -> 散らした種類の名前 {
        散らした種類の名前::綴りから生成する(match self {
            Self::酒場宿屋 => "酒場宿屋",
            Self::樫の木 => "樫の木",
            Self::一間四方の骨格 => "一間四方の骨格",
        })
    }

    /// その規則が指す部品の一覧。**規則と部品の一覧は常に対で使うため、種類が両方を答える。**
    /// 宣言の側が2つを別々に書くと、規則が指す部品を一覧へ書き忘れた宣言が型を通り、
    /// 「カタログに無い部品」という遠い場所の失敗になる。
    pub(super) fn 部品一覧(self) -> &'static [原型の識別] {
        match self {
            Self::酒場宿屋 => 酒場宿屋の部品一覧,
            Self::樫の木 => 樫の木の部品一覧,
            Self::一間四方の骨格 => 一間四方の骨格の部品一覧,
        }
    }

    /// その種類の規則と、展開の種の起点を組む。
    pub(super) fn 規則と種の起点を組む(self) -> Result<(組み立て規則, 生成の種), String> {
        match self {
            Self::酒場宿屋 => Ok((house_rule::酒場宿屋の規則()?, house_rule::家の並びの展開の種())),
            Self::樫の木 => Ok((tree_rule::樫の木の規則()?, tree_rule::木の並びの展開の種())),
            Self::一間四方の骨格 => Ok((frame_rule::一間四方の骨格の規則を組む()?, frame_rule::一間四方の骨格の並びの展開の種())),
        }
    }
}
