//! 画素の幾何: 読み戻し画像の寸法と、その中の画素の位置を表す型。
//!
//! 裸の`usize`で持つと、幅と高さ・横と縦を取り違えても型が通る。取り違えた添字は範囲の内側に
//! 落ちることが多く、そのときは落ちずに別の画素を読む。読んだ絵で判定が通ってしまうため、
//! 破れが判定の結果として現れない。数学DDDのとおり、次元(横方向か縦方向か)を型で分ける。
//!
//! 包みはすべて`#[repr(transparent)]`の`usize`であり、実行時の費用は無い。
//! 演算は必要な口だけを出す。任意の足し引きを許すと、位置と長さの区別がまた消えるためである。

/// 画素数で数えた画像の横の長さ。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 画像の幅(usize);

/// 画素数で数えた画像の縦の長さ。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 画像の高さ(usize);

/// 左端から数えた画素の横の位置。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 画素の横位置(usize);

/// 上端から数えた画素の縦の位置。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 画素の縦位置(usize);

/// 先頭から数えた画素の番号。行を上から、行の中を左から数える並びである。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 画素の番号(usize);

impl 画像の幅 {
    pub const fn 生成する(画素数: usize) -> Self {
        Self(画素数)
    }

    /// 画素数としての値。割合や千分率から位置を求める計算がこれを読む。
    pub const fn 画素数(self) -> usize {
        self.0
    }

    /// 左端から右端までの横の位置の並び。全画素を走査する判定がこれを使う。
    pub fn 位置一覧(self) -> impl Iterator<Item = 画素の横位置> {
        (0..self.0).map(画素の横位置::生成する)
    }
}

impl 画像の高さ {
    pub const fn 生成する(画素数: usize) -> Self {
        Self(画素数)
    }

    pub const fn 画素数(self) -> usize {
        self.0
    }

    /// 上端から下端までの縦の位置の並び。
    pub fn 位置一覧(self) -> impl Iterator<Item = 画素の縦位置> {
        (0..self.0).map(画素の縦位置::生成する)
    }
}

impl 画素の横位置 {
    pub const fn 生成する(位置: usize) -> Self {
        Self(位置)
    }

    pub const fn 値(self) -> usize {
        self.0
    }
}

impl 画素の縦位置 {
    pub const fn 生成する(位置: usize) -> Self {
        Self(位置)
    }

    pub const fn 値(self) -> usize {
        self.0
    }
}

impl 画素の番号 {
    pub const fn 生成する(番号: usize) -> Self {
        Self(番号)
    }

    pub const fn 値(self) -> usize {
        self.0
    }
}
