//! 空間に属する慣性テンソル(重心まわりの慣性の対称正定値3×3行列)。主軸と主慣性から I = P J Pᵀ で組み、角速度を角運動量へ写す。
//! ジャイロ項を陰的に1段解く計算もここが持つ。行列の演算はこの型の内側に閉じ、公開する署名は角速度・角運動量・秒だけである。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断4」「判断8」。

use std::fmt;
use std::marker::PhantomData;

use glam::Mat3;

use super::angular_impulse::回転衝撃;
use super::angular_velocity::角速度;
use super::rotation::クォータニオン;
use super::space::空間;
use crate::units::{キログラム平方メートル, 秒};

#[repr(transparent)]
pub struct 慣性テンソル<空間種> {
    内部: Mat3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 慣性テンソル<空間種> {
    /// 主軸(主軸座標からこの空間への回転)と主慣性の3値から I = P diag(J) Pᵀ を組む。
    pub fn 主軸と主慣性から生成する<軸空間: 空間>(
        主軸: &クォータニオン<軸空間, 空間種>,
        主慣性: [キログラム平方メートル; 3],
    ) -> Self {
        let 回転 = Mat3::from_quat(主軸.内部クォータニオン());
        let 対角 = Mat3::from_diagonal(glam::Vec3::new(主慣性[0].値(), 主慣性[1].値(), 主慣性[2].値()));
        Self::内部から生成する(回転 * 対角 * 回転.transpose())
    }

    /// 角運動量 L = I ω。
    pub fn 角速度を角運動量へ写す(&self, 角速度: 角速度<空間種>) -> 回転衝撃<空間種> {
        回転衝撃::内部から生成する(self.内部 * 角速度.内部ベクトル())
    }

    /// ジャイロ項を後退オイラー法で離散化した式 f(ω₁) = I (ω₁ − ω₀) + h ω₁ × (I ω₁) = 0 を、ω₀ でのニュートン法1回で解いた ω₁。
    /// ヤコビ行列は Jf = I + h ([ω₀]× I − [I ω₀]×) であり、ω₁ = ω₀ − Jf⁻¹ (h ω₀ × (I ω₀)) である。
    /// 慣性が定数である局所座標の角速度で呼ぶ。陽的に足すと高速回転で発散し、省くと中間軸の反転が再現されない(判断8)。
    pub fn ジャイロ項を陰的に一段解く(&self, 角速度: 角速度<空間種>, 刻み幅: 秒) -> 角速度<空間種> {
        let ω = 角速度.内部ベクトル();
        let h = 刻み幅.値();
        let 角運動量 = self.内部 * ω;
        let 残差 = h * ω.cross(角運動量);
        let ヤコビ = self.内部 + h * (外積の行列(ω) * self.内部 - 外積の行列(角運動量));
        角速度::内部から生成する(ω - ヤコビ.inverse() * 残差)
    }

    /// ジャイロ項を前進オイラー法で陽的に足した ω₁ = ω₀ − h I⁻¹ (ω₀ × (I ω₀))。剛体の予測の正典は陰的1段であり、この形は主軸の外の高速回転で
    /// 発散することの反証(判断8)だけが読む。
    pub fn ジャイロ項を陽的に一段足す(&self, 角速度: 角速度<空間種>, 刻み幅: 秒) -> 角速度<空間種> {
        let ω = 角速度.内部ベクトル();
        角速度::内部から生成する(ω - 刻み幅.値() * (self.内部.inverse() * ω.cross(self.内部 * ω)))
    }

    pub(crate) fn 内部から生成する(内部: Mat3) -> Self {
        Self {
            内部, _空間: PhantomData
        }
    }
}

// a との外積を表す3×3行列 [a]×。[a]× v = a × v である。
fn 外積の行列(a: glam::Vec3) -> Mat3 {
    Mat3::from_cols(
        glam::Vec3::new(0.0, a.z, -a.y),
        glam::Vec3::new(-a.z, 0.0, a.x),
        glam::Vec3::new(a.y, -a.x, 0.0),
    )
}

// 手動実装: deriveは幻影型パラメータ自身にも境界を要求するが、空間種は実行時表現を持たない。
impl<空間種> Clone for 慣性テンソル<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 慣性テンソル<空間種> {}

impl<空間種> PartialEq for 慣性テンソル<空間種> {
    fn eq(&self, 相手: &Self) -> bool {
        self.内部 == 相手.内部
    }
}

impl<空間種> fmt::Debug for 慣性テンソル<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("慣性テンソル").field("内部", &self.内部).finish()
    }
}
