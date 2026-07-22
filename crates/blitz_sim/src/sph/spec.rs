//! 弱圧縮性SPHの検証済み仕様。

use super::error::Sph仕様エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sph仕様 {
    平滑化半径: f32,
    静止密度: f32,
    圧力剛性: f32,
    粘性係数: f32,
    時間刻み: f32,
    重力加速度: [f32; 3],
    境界: [[f32; 3]; 2],
    反発係数: f32,
}

impl Sph仕様 {
    pub fn 生成する(
        流体定数: [f32; 5], 重力加速度: [f32; 3], 境界: [[f32; 3]; 2], 反発係数: f32
    ) -> Result<Self, Sph仕様エラー> {
        let [平滑化半径, 静止密度, 圧力剛性, 粘性係数, 時間刻み] = 流体定数;
        if 重力加速度.iter().any(|成分| !成分.is_finite()) {
            return Err(Sph仕様エラー::粒子ベクトルが不正 { 項目: "重力加速度" });
        }
        for (項目, 値) in [
            ("平滑化半径", 平滑化半径),
            ("静止密度", 静止密度),
            ("圧力剛性", 圧力剛性),
            ("時間刻み", 時間刻み),
        ] {
            if !値.is_finite() || 値 <= 0.0 {
                return Err(Sph仕様エラー::正値が必要 { 項目, 指定値: 値 });
            }
        }
        if !粘性係数.is_finite() || 粘性係数 < 0.0 {
            return Err(Sph仕様エラー::粘性係数が不正 { 指定値: 粘性係数 });
        }
        if !反発係数.is_finite() || !(0.0..=1.0).contains(&反発係数) {
            return Err(Sph仕様エラー::反発係数が不正 { 指定値: 反発係数 });
        }
        for (軸, (&最小, &最大)) in 境界[0].iter().zip(&境界[1]).enumerate() {
            if !最小.is_finite() || !最大.is_finite() || 最小 >= 最大 {
                return Err(Sph仕様エラー::境界が不正 { 軸 });
            }
        }
        Ok(Self {
            平滑化半径,
            静止密度,
            圧力剛性,
            粘性係数,
            時間刻み,
            重力加速度,
            境界,
            反発係数,
        })
    }

    pub(crate) fn 平滑化半径(&self) -> f32 {
        self.平滑化半径
    }
    pub(crate) fn 静止密度(&self) -> f32 {
        self.静止密度
    }
    pub(crate) fn 圧力剛性(&self) -> f32 {
        self.圧力剛性
    }
    pub(crate) fn 粘性係数(&self) -> f32 {
        self.粘性係数
    }
    pub(crate) fn 時間刻み(&self) -> f32 {
        self.時間刻み
    }
    pub(crate) fn 重力加速度(&self) -> [f32; 3] {
        self.重力加速度
    }
    pub(crate) fn 境界(&self) -> [[f32; 3]; 2] {
        self.境界
    }
    pub(crate) fn 反発係数(&self) -> f32 {
        self.反発係数
    }
}
