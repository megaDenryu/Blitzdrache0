//! 質量と長さの積の単位型。XPBDの位置の拘束のラグランジュ乗数がこの次元を持ち、剛体の逆慣性が腕と向きの外積に掛けて回転へ写すときの大きさがこれである。
//! 符号は任意である(縮める向きの補正で負になる)ため検証を持たない。

/// キログラムメートル単位の量。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct キログラムメートル(f32);

impl キログラムメートル {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}
