//! 静的物理形状: 建物のように動かない物体が、問い合わせに答えるために持つ幾何と識別である。
//!
//! 初版の物理の正本は、部品のglTFのアクセサ宣言から読む境界箱とする。メッシュの三角形からは導かない。
//! 描画資産から独立した物理形状の指定は物理アセット変換(#19)で導入する。この層が受け取るのは、
//! その境界箱からアセットコンパイラが焼いた結果だけである。
//!
//! 実装順4が持たせたのは幾何(任意姿勢の直方体)と識別(親の衝突対象と子形状)だけである。判断10の宣言面
//! (衝突分類・センサー性・物性参照・表裏性・親から見た局所姿勢の完全な形)は未実装であり、足すときは実行時形式の版上げを伴う。
//!
//! **形状に対する判定はここに無い。** 線分・カプセル・凸形状との交差の数学は`blitz_collision`が一意に所有する。
//! この層が受け持つのは、その数学へ渡せる形が実行中に存在し、チャンクの読込と解除に合わせて出入りすることまでである。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断10: 衝突形状が宣言できる属性」

mod child_shape;
mod child_shape_identifier;
mod chunk_shapes;
mod collision_object;
mod error;
mod half_extent;
mod object_identifier;
mod oriented_box;
mod storage;

#[cfg(test)]
pub(crate) mod shape_fixture;
#[cfg(test)]
mod shape_tests;
#[cfg(test)]
mod storage_tests;

pub use child_shape::衝突対象の子形状;
pub use child_shape_identifier::子形状識別子;
pub use chunk_shapes::チャンクの静的物理形状;
pub use collision_object::静的な衝突対象;
pub use error::静的物理形状エラー;
pub use half_extent::直方体の軸ごとの半分の長さ;
pub use object_identifier::衝突対象の安定識別子;
pub use oriented_box::任意姿勢の直方体;
pub use storage::チャンクごとの静的物理形状の保管;
