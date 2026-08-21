//! 1つの世界のソース一式を、カタログ・チャンク目録・生成台帳を含む実行時形式へ焼く編成。
//! CLIとエディターが同じ増分コンパイルを直接呼べるよう、処理本体をライブラリが所有する。

mod archetype_identity;
mod bake_report;
mod catalog;
mod chunk_ledger;
mod chunk_world;
mod compilation;
mod compilation_specification;
mod compile_target;
mod distant_terrain;
mod height_field;
mod instance_tally;
mod placed_instance_tally;
mod source_kind;
mod source_location;
mod world;

pub use compilation::実行時アセットのコンパイル;
pub use compilation_specification::{焼く世界の指定, 焼く世界の指定組み立て器};
pub use world::{一間四方の骨格の並びの種類, 家の並びの規模, 対象世界, 診断の原型};
