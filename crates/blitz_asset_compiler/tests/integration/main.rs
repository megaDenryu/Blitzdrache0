//! アセットコンパイラの統合試験一式。担当するのは、`mod`の列でこのフォルダの試験を1本の実行ファイルへ束ねることだけである。
//!
//! `tests/`の直下へ.rsファイルを置くと、Cargoはその1ファイルごとに独立した実行ファイルを組み立て、
//! 実行ファイル1本ごとに外部の部品すべてぶんのデバッグ情報を付ける。ファイルを責務で分ける原則を保ったまま
//! 実行ファイルを1本に保つため、分割の単位をこのフォルダにしている(参照: CLAUDE.md「ファイル・関数の分割」)。
//!
//! 高さ場の往復の共通の段取りは`height_field_roundtrip`が、複数材質の試験材料の組み立ては
//! `multi_material_fixture`が持つ。
//!
//! 束ねる前は試験のファイル1つ1つが実行ファイルの根であり、各ファイルの先頭に書いた`#![allow(non_snake_case)]`が
//! その実行ファイル全体に効いていた。束ねた後はこのファイルが根になるため、同じ緩めをここへ置く。
//! 日本語の識別子に大文字のASCIIが混じるとRustの命名規則の警告が出るためである。

#![allow(non_snake_case)]

mod chunk_world_roundtrip;
mod editor_world_directory_contract;
mod fox_computed_attributes_roundtrip;
mod fox_runtime_format_roundtrip;
mod fox_skin_roundtrip;
mod fox_walk_skin_not_collapsed;
mod height_field_catalog_lookup;
mod height_field_roundtrip;
mod multi_material_fixture;
mod multi_material_roundtrip;
mod shadow_scene_roundtrip;
mod smoke_asset_roundtrip;
mod surface_layer_texture_set_catalog_lookup;
mod surface_layer_texture_set_roundtrip;
mod vegetation_roundtrip;
