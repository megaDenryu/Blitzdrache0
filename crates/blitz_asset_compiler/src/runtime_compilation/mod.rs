//! 1つの世界のソース一式を、カタログ・チャンク目録・生成台帳を含む実行時形式へ焼く編成。
//! CLIとエディターが同じ増分コンパイルを直接呼べるよう、処理本体をライブラリが所有する。

mod archetype_identity;
mod bake_report;
mod building_outline_catalog;
mod catalog;
mod chunk_ledger;
mod chunk_world;
mod compilation;
mod compilation_specification;
mod compile_target;
mod distant_terrain;
mod error;
mod height_field;
mod instance_tally;
mod placed_instance_tally;
mod source_kind;
mod source_location;
mod surface_layer_texture_set;
mod vegetation_definition;
mod world;

pub use building_outline_catalog::{
    ベイ構造, 建物の入口方向, 建物の外接箱, 建物外形カタログ, 建物外形カタログのファイル, 建物外形カタログの組み立て係, 建物外形カタログエラー,
    建物定義, 建物定義ID, 建物定義の用途, 建物定義の識別子一覧, 空の建物外形カタログを作る,
};
pub(crate) use building_outline_catalog::{建物定義の正本, 識別子から建物定義を参照する};
pub use compilation::実行時アセットのコンパイル;
pub use compilation_specification::{焼く世界の指定, 焼く世界の指定組み立て器};
pub use error::実行時アセットのコンパイルエラー;
pub(crate) use vegetation_definition::{全植生定義, 植生の実体, 識別子から植生定義を参照する};
/// 骨格方式の部品の綴りと部品の一覧。エディターチャンクの焼き込みが格子から手順を作るために使う。
pub(crate) use world::assembly_rule_choice::{骨格方式の全部品の一覧, 骨格方式の部品の綴りを組む};
pub use world::{一間四方の骨格の並びの種類, 家の並びの規模, 対象世界, 診断の原型};
