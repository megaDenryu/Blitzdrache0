//! 角度の単位型。度からの変換コンストラクタを持つ。

use std::fmt::{Display, Formatter, Result as 書式の結果};
use std::ops::{Add, Mul, Sub};

/// ラジアン単位の角度。生値の取り出しは境界（GPU・外部API）専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ラジアン(f32);

impl ラジアン {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 度単位の角度からラジアンを生成する。
    pub fn 度から(度: f32) -> Self {
        Self(度.to_radians())
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }

    /// 一周の中で正負の半周へ畳んだ角。2つの向きの差をこの形にすると、回る側が短い弧に決まる。
    /// ちょうど半周の差は正の側の弧を返す。どちらの弧も同じ長さであり、片方へ決めておかないと
    /// 同じ入力の描画が実行ごとに逆へ回る。
    pub fn 一周の中で最短の弧へ畳む(self) -> Self {
        let 一周 = std::f32::consts::TAU;
        let 畳んだ角 = self.0.rem_euclid(一周);
        Self(if 畳んだ角 > 一周 / 2.0 {
            畳んだ角 - 一周
        } else {
            畳んだ角
        })
    }
}

impl Add for ラジアン {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for ラジアン {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f32> for ラジアン {
    type Output = Self;
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}

// 型付きエラーの文がオーナーの読む文として成立するために要る。単位を型で持った結果、読めない文しか出せなくなってはならない。
impl Display for ラジアン {
    fn fmt(&self, 書き出し先: &mut Formatter<'_>) -> 書式の結果 {
        write!(書き出し先, "{}ラジアン", self.0)
    }
}
