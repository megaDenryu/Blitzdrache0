//! TRS(平行移動・回転・スケール)からの変換合成。ジョイント階層のローカル変換組み立てに使う。

use glam::{Mat4, Vec3};

use super::position::位置;
use super::rotation::クォータニオン;
use super::space::空間;
use super::transform::変換;

impl<入力空間: 空間, 出力空間: 空間> 変換<入力空間, 出力空間> {
    /// 平行移動・回転・スケールから変換を合成する(判断43)。
    /// スケールは軸ごとの比率。均一スケールは全成分に同じ値を渡す。
    pub fn trsから生成する(
        平行移動: 位置<出力空間>, 回転: クォータニオン<入力空間, 出力空間>, スケール: [f32; 3]
    ) -> Self {
        let 平行移動行列 = Mat4::from_translation(平行移動.内部ベクトル());
        let 回転行列 = Mat4::from_quat(回転.内部クォータニオン());
        let スケール行列 = Mat4::from_scale(Vec3::from_array(スケール));
        変換::内部から生成する(平行移動行列 * 回転行列 * スケール行列)
    }
}
