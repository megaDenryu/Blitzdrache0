//! 個体1件ぶんの変換と、それに対応する法線行列。頂点シェーダーが`SV_InstanceID`で1件を参照する単位である。
//! 個体が1体だけの対象も1件の列としてこの内容を並べるため、単一個体のための別の配置は存在しない。
//! 注意: この構造の並びはシェーダーの`InstanceTransform`と1対1で対応する。並びを変えると全対象が壊れた行列で描かれる。

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

    /// 描画対象1つぶんの全個体の変換をGPUへ載せる形へ直す。1件でも複数件でも同じ経路を通るため、
    /// 個体数で確保の仕方が変わらない。
    pub(crate) fn 一覧を作る(変換一覧: &[変換<ローカル, ワールド>]) -> Result<Vec<Self>, レンダラーエラー> {
        変換一覧.iter().map(|変換| Self::変換から作る(*変換)).collect()
    }
}
