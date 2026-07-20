//! 空間から空間への変換（同次変換行列）。合成・適用・GPU境界出口を提供する。

use std::fmt;
use std::marker::PhantomData;

use glam::Mat4;

use super::position::位置;
use super::space::空間;

/// `入力空間`から`出力空間`への変換。型が合うときのみ合成・適用がコンパイルでき、
/// 座標系の混同・行列の掛け順ミスを型検査で防ぐことがこの型の存在意義。
#[repr(transparent)]
pub struct 変換<入力空間, 出力空間> {
    内部: Mat4,
    _入力: PhantomData<入力空間>,
    _出力: PhantomData<出力空間>,
}

impl<入力空間, 出力空間> 変換<入力空間, 出力空間> {
    pub(crate) fn 内部から生成する(内部: Mat4) -> Self {
        Self {
            内部,
            _入力: PhantomData,
            _出力: PhantomData,
        }
    }

    /// 変換を合成する: `self`（入力→出力）の後に`先`（出力→さらに先）を適用した
    /// 変換（入力→さらに先）を返す。出力空間が一致するときだけコンパイルが通る。
    pub fn 合成する<さらに先>(&self, 先: 変換<出力空間, さらに先>) -> 変換<入力空間, さらに先> {
        変換::内部から生成する(先.内部 * self.内部)
    }

    /// GPU境界向けの唯一の生値出口: 列優先の4x4配列。
    pub fn gpu境界用列優先配列(&self) -> [[f32; 4]; 4] {
        self.内部.to_cols_array_2d()
    }

    /// 外部データ(アセット読込・GPU境界)由来の列優先4x4行列から変換を作る。
    /// glTFの逆バインド行列等、既に行列として渡されるデータの取り込み口(判断42)。
    pub fn 列優先配列から生成する(列優先: [[f32; 4]; 4]) -> Self {
        変換::内部から生成する(Mat4::from_cols_array_2d(&列優先))
    }
}

impl<入力空間: 空間, 出力空間: 空間> 変換<入力空間, 出力空間> {
    /// 位置を写す。透視投影を含む変換では、GPU実行時と同じ規約で
    /// 透視除算まで行った座標を返す（境界外でも常に安全に呼べるようにするため）。
    pub fn 適用する(&self, 位置: 位置<入力空間>) -> 位置<出力空間> {
        let 変換後 = self.内部.project_point3(位置.内部ベクトル());
        位置::内部から生成する(変換後)
    }
}

// 手動実装する理由: 幻影型パラメータにClone/Copy境界を要求しないため。
impl<入力空間, 出力空間> Clone for 変換<入力空間, 出力空間> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<入力空間, 出力空間> Copy for 変換<入力空間, 出力空間> {}

impl<入力空間, 出力空間> PartialEq for 変換<入力空間, 出力空間> {
    fn eq(&self, other: &Self) -> bool {
        self.内部 == other.内部
    }
}

impl<入力空間, 出力空間> fmt::Debug for 変換<入力空間, 出力空間> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("変換").field("内部", &self.内部).finish()
    }
}
