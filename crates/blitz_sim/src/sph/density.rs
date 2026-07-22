//! 全粒子対走査によるSPH密度のCPU参照計算。

use super::kernel::密度核;
use super::particle::流体粒子;
use super::spec::Sph仕様;

pub fn sph密度を計算する(粒子一覧: &[流体粒子], 仕様: &Sph仕様) -> Vec<f32> {
    粒子一覧
        .iter()
        .map(|対象| {
            粒子一覧
                .iter()
                .map(|近傍| {
                    let 距離二乗 = 距離二乗(対象.位置(), 近傍.位置());
                    近傍.質量() * 密度核(距離二乗, 仕様.平滑化半径())
                })
                .sum()
        })
        .collect()
}

pub(crate) fn 距離二乗(a: [f32; 3], b: [f32; 3]) -> f32 {
    let 差 = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    差[0] * 差[0] + 差[1] * 差[1] + 差[2] * 差[2]
}
