//! 組み立て規則の種類ごとの部品の一覧の台帳と、種類からその一覧を選ぶ引き当て。
//!
//! 台帳を親から分けるのは、規則の種類が増えるたびにこの一覧だけが伸び、規則の引き当てと押し合うためである。
//! 世界のソースディレクトリ名の台帳を`directory_source_path`が持つのと同じ分け方である。
//!
//! 原型はすべて外部のアセットリポジトリの`parts/`から引く。部品IDはglTFのファイル名から決まるため、
//! この台帳が持つのはファイル名と安定IDの対応だけである。

use super::super::super::archetype_identity::原型の識別;
use super::部品の組み立て規則の種類;
use crate::ソースアセットの相対パス;

/// 台帳の1行が書いた2つの綴りを`原型の識別`へ組み上げる。表の1行を1行に収めたまま、
/// ソースの綴りが相対パスの型へ入る口をこの1箇所に保つ。
const fn 部品(安定id: &'static str, ソースの綴り: &'static str) -> 原型の識別 {
    原型の識別::生成する(安定id, ソースアセットの相対パス::生成する(ソースの綴り))
}

/// 酒場宿屋を組み立てる部品の一覧。原型はすべて外部のアセットリポジトリの`parts/`から引く。
/// 部品IDはglTFのファイル名から決まるため、この表はファイル名と安定IDの対応だけを持つ。
const 酒場宿屋の部品一覧: &[原型の識別] = &[
    部品("part_tavern_f1_base", "parts/Mod_Tavern_F1_Base.glb"),
    部品("part_tavern_f2_jetty", "parts/Mod_Tavern_F2_Jetty.glb"),
    部品("part_tavern_f3_jetty", "parts/Mod_Tavern_F3_Jetty.glb"),
    部品("part_tavern_roof_gable", "parts/Mod_Tavern_Roof_Gable.glb"),
    部品("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
    部品("part_tavern_dormer_roof", "parts/Mod_Tavern_Dormer_Roof.glb"),
    部品("part_tavern_chimney", "parts/Mod_Tavern_Chimney.glb"),
];

/// 樫の木を組み立てる部品の一覧。幹の一節を何節積むかは規則の仕事であり、この表は部品の種類を1件ずつ持つだけである。
const 樫の木の部品一覧: &[原型の識別] = &[
    部品("part_tree_oak_trunk_segment", "parts/Mod_Tree_Oak_Trunk_Segment.glb"),
    部品("part_tree_oak_branch_large", "parts/Mod_Tree_Oak_Branch_Large.glb"),
    部品("part_tree_oak_foliage_cluster", "parts/Mod_Tree_Oak_Foliage_Cluster.glb"),
];

/// 一間四方の骨格を組み立てる部品の一覧。骨格1件と壁3種と床板1件と出窓1件である。
/// 壁3種のどれにも必ず入れる面を割り当ててあるため、その4件はどの種でも据わる。
///
/// **床板と出窓を載せるのは、骨格が床のはめ口を、壁3種が外面の飾り取付を宣言しているためである。**
/// カタログの組み上げは接合種別の対の相手がカタログに在ることを課すため、どちらかを外すと組み上げが落ちる。
/// これらの世界の規則は床も飾りも置かないため、2件は据わらず群も作られない。
/// **据えた部品の種類数は4のままであり、材質スロットの総和も8のままである。**
const 一間四方の骨格の部品一覧: &[原型の識別] = &[
    部品("part_frame_bay_single", "parts/Mod_Frame_Bay_Single.glb"),
    部品("part_wall_halftimber_solid", "parts/Mod_Wall_HalfTimber_Solid.glb"),
    部品("part_wall_halftimber_window", "parts/Mod_Wall_HalfTimber_Window.glb"),
    部品("part_wall_halftimber_doorframe", "parts/Mod_Wall_HalfTimber_DoorFrame.glb"),
    部品("part_frame_floor", "parts/Mod_Frame_Floor.glb"),
    部品("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
];

/// 屋根つきの家を組み立てる部品の一覧。骨格方式の6件へ切妻屋根を足した7件である。
/// **床を張るのは家2件とも、出窓を差し込むのは1ベイの家だけである。** 1ベイの家は7種類が据わって材質スロットの
/// 総和が15(1・2・3・2・1・2・4)、2段の家は出窓が据わらないため6種類で総和11になる。
const 屋根つきの家の部品一覧: &[原型の識別] = &[
    部品("part_frame_bay_single", "parts/Mod_Frame_Bay_Single.glb"),
    部品("part_wall_halftimber_solid", "parts/Mod_Wall_HalfTimber_Solid.glb"),
    部品("part_wall_halftimber_window", "parts/Mod_Wall_HalfTimber_Window.glb"),
    部品("part_wall_halftimber_doorframe", "parts/Mod_Wall_HalfTimber_DoorFrame.glb"),
    部品("part_frame_floor", "parts/Mod_Frame_Floor.glb"),
    部品("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
    部品("part_frame_roof_gable", "parts/Mod_Frame_Roof_Gable.glb"),
];

/// 煙突を付けた家を組み立てる部品の一覧。屋根つきの家の7件へ煙突セグメントを足した8件である。
/// **煙突は1つの部品を2段積むため、部品の種類は1件しか増えない。** 材質を1つしか持たないため、
/// 材質スロットの総和は15から16へ1つだけ増える。
const 煙突を付けた家の部品一覧: &[原型の識別] = &[
    部品("part_frame_bay_single", "parts/Mod_Frame_Bay_Single.glb"),
    部品("part_wall_halftimber_solid", "parts/Mod_Wall_HalfTimber_Solid.glb"),
    部品("part_wall_halftimber_window", "parts/Mod_Wall_HalfTimber_Window.glb"),
    部品("part_wall_halftimber_doorframe", "parts/Mod_Wall_HalfTimber_DoorFrame.glb"),
    部品("part_frame_floor", "parts/Mod_Frame_Floor.glb"),
    部品("part_tavern_oriel_f2", "parts/Mod_Tavern_Oriel_F2.glb"),
    部品("part_frame_roof_gable", "parts/Mod_Frame_Roof_Gable.glb"),
    部品("part_frame_chimney_segment", "parts/Mod_Frame_Chimney_Segment.glb"),
];

/// ベイ格子が組み立てに使う8つの役割の部品の一覧。煙突を付けた家の一覧と同じ8件であり、
/// 役割の数え上げ(骨格・壁3種・切妻屋根・床板・出窓・煙突セグメント)と1対1に対応する。
/// 別の並びを新しく書かないのは、同じ8件の台帳が2つになると片方だけを直した食い違いが焼くまで出ないためである。
pub(crate) fn 骨格方式の全部品の一覧() -> &'static [原型の識別] {
    煙突を付けた家の部品一覧
}

pub(super) fn 部品一覧を選ぶ(種類: 部品の組み立て規則の種類) -> &'static [原型の識別] {
    match 種類 {
        部品の組み立て規則の種類::酒場宿屋 => 酒場宿屋の部品一覧,
        部品の組み立て規則の種類::樫の木 => 樫の木の部品一覧,
        部品の組み立て規則の種類::一間四方の骨格 | 部品の組み立て規則の種類::二段に積んだ骨格 | 部品の組み立て規則の種類::横に二ベイ継いだ骨格 => {
            一間四方の骨格の部品一覧
        }
        部品の組み立て規則の種類::屋根を載せた一間四方の家 | 部品の組み立て規則の種類::屋根を載せた二段の家 => {
            屋根つきの家の部品一覧
        }
        部品の組み立て規則の種類::煙突を付けた一間四方の家 => 煙突を付けた家の部品一覧,
    }
}
