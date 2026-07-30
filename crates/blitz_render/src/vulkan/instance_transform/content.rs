//! 個体1件ぶんの変換と、それに対応する法線行列。頂点シェーダーが`SV_InstanceID`で1件を参照する単位である。
//! 描画対象ユニフォームの先頭もこの内容を持つため、個体が1体だけの対象は専用バッファを確保せずユニフォームの先頭を指す。
//! 注意: この構造の並びを変えると描画対象ユニフォームの先頭と食い違い、単一個体の対象が壊れた行列で描かれる。

use blitz_math::{ローカル, ワールド, 変換};
use glam::{Mat3, Mat4};

use crate::error::レンダラーエラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 個体変換内容 {
    pub(crate) ローカルからワールド: [[f32; 4]; 4],
    pub(crate) 法線ローカルからワールド: [[f32; 3]; 3],
}

impl 個体変換内容 {
    /// 法線行列は変換の線形部の逆転置である。行列式が0に近い変換は法線を求められないため、無言で単位行列へ落とさず型付きエラーにする。
    pub(crate) fn 変換から作る(ローカルからワールド: 変換<ローカル, ワールド>) -> Result<Self, レンダラーエラー> {
        let 生行列 = ローカルからワールド.gpu境界用列優先配列();
        let 法線基底 = Mat3::from_mat4(Mat4::from_cols_array_2d(&生行列));
        let 行列式 = 法線基底.determinant();
        if !行列式.is_finite() || 行列式.abs() <= f32::EPSILON {
            return Err(レンダラーエラー::描画対象変換非可逆);
        }
        Ok(Self {
            ローカルからワールド: 生行列,
            法線ローカルからワールド: 法線基底.inverse().transpose().to_cols_array_2d(),
        })
    }
}
