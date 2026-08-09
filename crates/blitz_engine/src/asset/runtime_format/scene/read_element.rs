//! 版をまたいで並びが変わらないシーン要素の読み取り。版ごとの組み立てが共通して使う。
//! 版で変わるのは描画対象の形状と、メッシュのプリミティブ列・材質の並びだけであり、行列・スキン・アニメーションの並びは変わらない。

mod animation;
mod instance;
mod material;
mod material_element;
mod material_set;
mod matrix;
mod mesh;
mod mesh_list;
mod mesh_primitive;
mod mesh_skin_vertex;
mod skin;
mod texture;

pub(super) use animation::一覧を読む as アニメーション一覧を読む;
pub(super) use instance::{境界を読む, 配置列を読む};
pub(super) use material::読む as マテリアルを読む;
pub(super) use material_set::読む as 材質集合を読む;
pub(super) use matrix::行列を読む;
pub(super) use mesh::{旧版を読む as 旧版のメッシュを読む, 読む as メッシュを読む};
pub(super) use mesh_list::読む as メッシュ列を読む;
pub(super) use skin::{読む as スキンを読む, 頂点属性を検査する};
pub(super) use texture::{
    テクスチャを読む工程, 版4までを読む as 版4までのテクスチャを読む, 版5を読む as 版5のテクスチャを読む
};

/// 版1から版3のメッシュ1つ分の最小バイト数。頂点数4 + 頂点1件48 + 添字数4 + 添字1件4 + スキン頂点属性の有無1の合計である。
pub(super) const メッシュ最小長: usize = 61;

/// 版4以降のメッシュ1つ分の最小バイト数。旧版の61へプリミティブ数4とプリミティブ1件16を足した合計である。
pub(super) const 版4以降のメッシュ最小長: usize = メッシュ最小長 + 4 + mesh_primitive::プリミティブ長;

/// 版1から版3で、描画対象のうち形状より前と後の最小バイト数。識別子8 + 所有チャンク8 + 行列64 + 材質31の合計である。
/// 版ごとの読み取りが宣言件数の境界検査に使うため、形状の並びだけを版が足す。
pub(super) const 形状以外の描画対象長: usize = 111;

/// 版4以降で、描画対象のうち形状より前と後の最小バイト数。識別子8 + 所有チャンク8 + 行列64 + 材質数4 + 材質1件39の合計である。
pub(super) const 版4以降の形状以外の描画対象長: usize = 8 + 8 + 64 + 4 + material_set::材質最小長;
