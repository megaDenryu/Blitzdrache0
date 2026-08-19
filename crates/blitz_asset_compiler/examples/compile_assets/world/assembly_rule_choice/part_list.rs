//! 組み立て規則の種類ごとの部品の一覧の台帳と、種類からその一覧を選ぶ引き当て。
//!
//! 台帳を親から分けるのは、規則の種類が増えるたびにこの一覧だけが伸び、規則の引き当てと押し合うためである。
//! 世界のソースディレクトリ名の台帳を`directory_source_path`が持つのと同じ分け方である。
//!
//! 原型はすべて外部のアセットリポジトリの`parts/`から引く。部品IDはglTFのファイル名から決まるため、
//! この台帳が持つのはファイル名と安定IDの対応だけである。

use super::super::super::archetype_identity::原型の識別;
use super::部品の組み立て規則の種類;

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

/// 屋根つきの家を組み立てる部品の一覧。骨格方式の4件へ切妻屋根を足した5件であり、材質スロットは1・2・3・2・2で総和10である。
const 屋根つきの家の部品一覧: &[原型の識別] = &[
    原型の識別::生成する("part_frame_bay_single", "parts/Mod_Frame_Bay_Single.glb"),
    原型の識別::生成する("part_wall_halftimber_solid", "parts/Mod_Wall_HalfTimber_Solid.glb"),
    原型の識別::生成する("part_wall_halftimber_window", "parts/Mod_Wall_HalfTimber_Window.glb"),
    原型の識別::生成する("part_wall_halftimber_doorframe", "parts/Mod_Wall_HalfTimber_DoorFrame.glb"),
    原型の識別::生成する("part_frame_roof_gable", "parts/Mod_Frame_Roof_Gable.glb"),
];

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
    }
}
