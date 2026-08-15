//! エンジンのglTF入力契約への適合検査。対象は.glbまたは.gltfの1ファイルであり、返すのは概要と全件の指摘である。1件目で止めない。
//!
//! ローダーの隣に置くのは、契約の正本がローダーの実装だからである。検査は`document::文書を開く`・`mesh::メッシュデータを取り出す`・
//! `material::マテリアルを取り出す`・`archetype_material::全段が同じ描画条件かを検査する`をそのまま呼び、読み方と規則の写しを持たない。
//! ローダーを変えた者がこの検査も同じ視野で直せるよう物理的に隣接させる。
//!
//! 検査する契約は2つある。1つは`ソースシーンを読み込む`の契約(1ファイル1メッシュ、そのメッシュは複数の材質スロットを持てる)であり、
//! もう1つは`原型ソースを読み込む`の契約(複数のメッシュを詳細段の列として読む)である。どちらで検査するかは呼び出し側が
//! `検査する契約`で渡し、文書の形から推し量らない。
//!
//! 入口は`target_file`の役割つきパスであり、集まった指摘と概要は`inspection`の型が所有する。走査の工程はその型のメソッドとして
//! 工程ごとのファイルにある。
//! 参照: `_doc/設計/Blenderアセット運用.md`

mod archetype_bounds;
mod archetype_detail_order_scan;
mod archetype_ground_origin_scan;
mod archetype_material_agreement;
mod archetype_node_scan;
mod archetype_primitive_scan;
mod archetype_scan;
mod document_scan;
mod finding;
mod geometry_scan;
mod inspected_contract;
mod inspection;
mod material_declaration_scan;
mod material_drop_scan;
mod material_scan;
mod mesh_data_check;
mod node_scan;
mod node_transform;
mod primitive_preservation;
mod primitive_scan;
mod result;
mod result_display;
mod skin_scan;
mod slot_materials;
mod target;
mod target_file;
mod texture_scan;

#[cfg(test)]
mod archetype_fixture;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod fixture_json;
#[cfg(test)]
mod glb_fixture;

pub use finding::{契約指摘, 重大度};
pub use inspected_contract::検査する契約;
pub use result::{契約検査概要, 契約検査結果};
pub use target_file::入力契約を検査するglTFのファイル;
