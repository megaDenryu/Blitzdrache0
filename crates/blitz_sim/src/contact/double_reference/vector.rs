//! 倍精度の参照計算が使う3成分のベクトル(Issue #59の数値契約の診断)。単位を持たない実数だけを扱い、単位の
//! 意味は呼び出し側の行と連立が持つ。単精度の側の`symmetric_system`が単位を持たないのと同じ切り分けである。
//! 単精度のドメイン型を倍精度へ広げる口をこの型が持つのは、広げる先が3成分の実数1種類しか無く、広げる綴りを
//! 1箇所へ閉じられるためである。広げるのに`as`を使わず`f64::from`を読むのは、単精度から倍精度への変換が
//! 値を1ビットも変えない拡大変換であることを型で示すためである。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

#![cfg(test)]

use blitz_math::{位置, 変位, 方向, 空間};

/// 倍精度の3成分のベクトル。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::contact) struct 倍精度の三次元ベクトル {
    x: f64,
    y: f64,
    z: f64,
}

impl 倍精度の三次元ベクトル {
    pub(in crate::contact) fn 成分から生成する(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(in crate::contact) fn 零() -> Self {
        Self::成分から生成する(0.0, 0.0, 0.0)
    }

    pub(in crate::contact) fn 単精度の位置から広げる<空間種: 空間>(単精度: 位置<空間種>) -> Self {
        Self::成分から生成する(f64::from(単精度.x().値()), f64::from(単精度.y().値()), f64::from(単精度.z().値()))
    }

    pub(in crate::contact) fn 単精度の変位から広げる<空間種: 空間>(単精度: 変位<空間種>) -> Self {
        Self::成分から生成する(f64::from(単精度.x().値()), f64::from(単精度.y().値()), f64::from(単精度.z().値()))
    }

    pub(in crate::contact) fn 単精度の方向から広げる<空間種: 空間>(単精度: 方向<空間種>) -> Self {
        Self::成分から生成する(f64::from(単精度.x()), f64::from(単精度.y()), f64::from(単精度.z()))
    }

    pub(in crate::contact) fn x(&self) -> f64 {
        self.x
    }

    pub(in crate::contact) fn y(&self) -> f64 {
        self.y
    }

    pub(in crate::contact) fn z(&self) -> f64 {
        self.z
    }

    pub(in crate::contact) fn 足す(&self, 相手: &Self) -> Self {
        Self::成分から生成する(self.x + 相手.x, self.y + 相手.y, self.z + 相手.z)
    }

    pub(in crate::contact) fn 引く(&self, 相手: &Self) -> Self {
        Self::成分から生成する(self.x - 相手.x, self.y - 相手.y, self.z - 相手.z)
    }

    pub(in crate::contact) fn 比で伸ばす(&self, 比: f64) -> Self {
        Self::成分から生成する(self.x * 比, self.y * 比, self.z * 比)
    }

    pub(in crate::contact) fn 逆を向く(&self) -> Self {
        self.比で伸ばす(-1.0)
    }

    pub(in crate::contact) fn 内積(&self, 相手: &Self) -> f64 {
        self.x * 相手.x + self.y * 相手.y + self.z * 相手.z
    }

    pub(in crate::contact) fn 外積(&self, 相手: &Self) -> Self {
        Self::成分から生成する(
            self.y * 相手.z - self.z * 相手.y,
            self.z * 相手.x - self.x * 相手.z,
            self.x * 相手.y - self.y * 相手.x,
        )
    }

    pub(in crate::contact) fn 長さ(&self) -> f64 {
        self.内積(self).sqrt()
    }

    /// この向きの成分を取り除いた残り v − (v·n) n。錨の接線変位を求める式が読む。
    pub(in crate::contact) fn 向きの成分を除く(&self, 向き: &Self) -> Self {
        self.引く(&向き.比で伸ばす(self.内積(向き)))
    }

    /// 長さ1へ正規化した向き。長さが零なら無しである(接線変位が零の点は連立へ入れない)。
    pub(in crate::contact) fn 単位方向(&self) -> Option<Self> {
        let 長さ = self.長さ();
        if 長さ > 0.0 { Some(self.比で伸ばす(1.0 / 長さ)) } else { None }
    }
}
