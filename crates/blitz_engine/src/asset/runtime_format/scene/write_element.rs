//! 版をまたいで並びが変わらないシーン要素の書き込み。版ごとの組み立てが共通して使う。
//! 読み取り側の`read_element`と対になっており、片方だけを変えると往復が壊れるため、要素の並びを決める箇所を2つに散らさない。

mod animation;
mod material;
mod matrix;
mod mesh;
mod skin;

pub(super) use animation::一覧を書く as アニメーション一覧を書く;
pub(super) use material::書く as マテリアルを書く;
pub(super) use matrix::行列を書く;
pub(super) use mesh::書く as メッシュを書く;
pub(super) use skin::書く as スキンを書く;
