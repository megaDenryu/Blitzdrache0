//! エンジンのglTF入力契約への適合検査。対象は.glbまたは.gltfの1ファイルであり、返すのは概要と全件の指摘である。1件目で止めない。
//!
//! ローダーの隣に置くのは、契約の正本がローダーの実装だからである。検査は`document::文書を開く`・`mesh::メッシュデータを取り出す`・
//! `material::マテリアルを取り出す`をそのまま呼び、読み方の写しを持たない。ローダーを変えた者がこの検査も同じ視野で直せるよう物理的に隣接させる。
//!
//! 検査するのは`ソースシーンを読み込む`の契約(1ファイル1メッシュ、そのメッシュは複数の材質スロットを持てる)である。植生の原型は複数メッシュをLODの段として読む別の契約を持ち、
//! この検査の対象ではない。原型の検査を足すときは`archetype_material`の段間比較を再利用する。
//!
//! 入口は`target_file`の役割つきパスであり、集まった指摘と概要は`inspection`の型が所有する。走査の工程はその型のメソッドとして
//! 工程ごとのファイルにある。
//! 参照: `_doc/設計/Blenderアセット運用.md`

mod document_scan;
mod finding;
mod geometry_scan;
mod inspection;
mod material_declaration_scan;
mod material_drop_scan;
mod material_scan;
mod mesh_data_check;
mod node_scan;
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
mod contract_tests;
#[cfg(test)]
mod fixture_json;
#[cfg(test)]
mod glb_fixture;

pub use finding::{契約指摘, 重大度};
pub use result::{契約検査概要, 契約検査結果};
pub use target_file::入力契約を検査するglTFのファイル;
