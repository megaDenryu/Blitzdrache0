//! 版をまたいで並びが変わらないシーン要素の書き込み。版ごとの組み立てが共通して使う。
//! 読み取り側の`read_element`と対になっており、片方だけを変えると往復が壊れるため、要素の並びを決める箇所を2つに散らさない。

mod animation;
mod instance;
#[cfg(test)]
mod material;
mod material_element;
mod material_set;
mod matrix;
mod mesh;
mod mesh_list;
mod mesh_primitive;
mod mesh_skin_vertex;
mod skin;

pub(super) use animation::一覧を書く as アニメーション一覧を書く;
pub(super) use instance::{境界を書く, 配置列を書く};
#[cfg(test)]
pub(super) use material::書く as マテリアルを書く;
pub(super) use material_set::書く as 材質集合を書く;
pub(super) use matrix::行列を書く;
#[cfg(test)]
pub(super) use mesh::旧版を書く as 旧版のメッシュを書く;
pub(super) use mesh::書く as メッシュを書く;
pub(super) use mesh_list::書く as メッシュ列を書く;
pub(super) use skin::書く as スキンを書く;
