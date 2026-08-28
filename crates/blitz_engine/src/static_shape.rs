//! 静的物理形状: 建物のように動かない物体が、問い合わせに答えるために持つ幾何と識別である。
//!
//! 描画に使うメッシュから導かず、部品カタログが持つ部品ごとの直方体を正本としてアセットコンパイラが焼いた結果だけを受け取る。
//! 物理形状と描画形状を同一視しないためである。持つのは幾何(任意姿勢の直方体)と識別(親の衝突対象と子形状)だけであり、
//! 衝突分類・物性・センサー性・表裏性はまだ持たない。属性の目録は設計正本が持ち、足すのは要求する形状が実物になったときである。
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
