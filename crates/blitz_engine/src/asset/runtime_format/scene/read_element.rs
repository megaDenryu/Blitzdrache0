//! 版をまたいで並びが変わらないシーン要素の読み取り。版ごとの組み立てが共通して使う。
//! 版で変わるのは描画対象の形状だけであり、行列・メッシュ・材質・スキン・アニメーションの並びは変わらないため、実装をここ1箇所が持つ。

mod animation;
mod instance;
mod material;
mod matrix;
mod mesh;
mod mesh_list;
mod skin;

pub(super) use animation::一覧を読む as アニメーション一覧を読む;
pub(super) use instance::{境界を読む, 配置列を読む};
pub(super) use material::読む as マテリアルを読む;
pub(super) use matrix::行列を読む;
pub(super) use mesh::読む as メッシュを読む;
pub(super) use mesh_list::読む as メッシュ列を読む;
pub(super) use skin::{読む as スキンを読む, 頂点属性を検査する};

/// メッシュ1つ分の最小バイト数。頂点数4 + 頂点1件48 + 添字数4 + 添字1件4 + スキン頂点属性の有無1の合計である。
pub(super) const メッシュ最小長: usize = 61;

/// 描画対象のうち形状より前と後の最小バイト数。識別子8 + 所有チャンク8 + 行列64 + 材質31の合計である。
/// 版ごとの読み取りが宣言件数の境界検査に使うため、形状の並びだけを版が足す。
pub(super) const 形状以外の描画対象長: usize = 111;
