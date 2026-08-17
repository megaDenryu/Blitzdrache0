//! どのチャンク世界を1つの出力ルートへ焼くかの選択。1つの出力ルートは1つのカタログと1つのチャンク目録を持つため、
//! 同じ座標を持つ2つの世界は同じ出力ルートへ同居できない。世界の選択がそのまま出力ルートの選択になる。
//! どのアセットを焼くかの宣言は`asset_declaration`が、定義1件の組み立ては`definition_kind`が持ち、
//! プロセス境界の綴りとその解析は`argument_name`が、世界のソースディレクトリ名は`directory_source_path`が、
//! チャンクのソース形式は`chunk_source_kind`が、地面へ散らす原型の一覧は`scatter_declaration`が持つ。宣言をコンパイラが受け取る指定へ写す手順は、
//! 小物群を`prop_group_declaration`が、目視見本を`visual_sample_declaration`が、地面へ据える固定物を`fixed_placement_declaration`が持ち、
//! 世界ごとの宣言は`fox_tour_declaration`と`stone_hut_declaration`と`night_lights_declaration`と`part_house_row_declaration`と
//! `part_tree_row_declaration`と`part_frame_row_declaration`にあり、部品で組んだ並びの宣言の型と規則は`part_row_declaration`が持つ。

mod argument_name;
pub(super) mod assembled_scatter_declaration;
mod assembly_rule_choice;
mod asset_declaration;
mod asset_definition_list;
mod chunk_source_kind;
mod definition_kind;
mod directory_source_path;
pub(super) mod fixed_placement_declaration;
mod fox_tour_assembled_scatter_declaration;
pub(super) mod fox_tour_declaration;
mod fox_tour_scatter_declaration;
mod night_lights_declaration;
pub(super) mod part_frame_row_declaration;
pub(super) mod part_house_row_declaration;
pub(super) mod part_row_declaration;
mod part_tree_row_declaration;
pub(super) mod prop_group_declaration;
mod provenance;
mod scatter_declaration;
pub(super) mod stone_hut_declaration;
mod vegetation_declaration;
mod vertex_diagnostic_declaration;
mod village_declaration;
pub(super) mod visual_sample_declaration;

mod target_world;

pub(super) use target_world::対象世界;
