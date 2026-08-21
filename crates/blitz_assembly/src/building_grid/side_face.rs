//! 升目の側面: 升目の4つの縦の面。1つの面が壁のはめ口とベイの継ぎ口を重ねて持ち、どちらか一方だけを使う。
//!
//! 面が2つの口の名前を答えるのは、同じ物理面へ2つの種別が重ねて宣言されているためである。面を選べば
//! 両方の綴りが決まるため、正面の壁を背面の継ぎ口へ繋ぐ取り違えを変換が書けない。
//!
//! 面に属さない接合点の綴り(`joint_spelling`)から分けるのは、変わる理由が違うためである。面と口の対応は
//! 骨格の形が変わると変わり、綴りはBlender側が名前を改めると変わる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「骨格の上下積層とベイの継ぎ口(2026-08-19 追加)」

use std::fmt;

use crate::joint::接合点名;

use super::joint_spelling::接合点名を作る;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum 升目の側面 {
    正面,
    背面,
    左面,
    右面,
}

/// 側面を走査する順序を型が1箇所で持つ。順序が変われば壁を据える順が変わり、同じ格子から違う手順が出る。
const 全側面: [升目の側面; 4] = [升目の側面::正面, 升目の側面::背面, 升目の側面::左面, 升目の側面::右面];

impl 升目の側面 {
    pub fn 全側面を数え上げる() -> [Self; 4] {
        全側面
    }

    /// 升目の宣言が側面ごとのはめ口の値を配列で持つため、側面から位置が一意に決まることが要る。
    pub(super) fn 添字(self) -> usize {
        match self {
            Self::正面 => 0,
            Self::背面 => 1,
            Self::左面 => 2,
            Self::右面 => 3,
        }
    }

    /// 隣の升目から見て、この面と同じ物理面にあたる側面。横へ継ぐときの子の側の口をこれが決める。
    pub(super) fn 向かい合う側面(self) -> Self {
        match self {
            Self::正面 => Self::背面,
            Self::背面 => Self::正面,
            Self::左面 => Self::右面,
            Self::右面 => Self::左面,
        }
    }

    pub(super) fn 壁のはめ口の接合点名(self) -> 接合点名 {
        接合点名を作る(match self {
            Self::正面 => "正面のはめ口",
            Self::背面 => "背面のはめ口",
            Self::左面 => "左面のはめ口",
            Self::右面 => "右面のはめ口",
        })
    }

    pub(super) fn ベイの継ぎ口の接合点名(self) -> 接合点名 {
        接合点名を作る(match self {
            Self::正面 => "正面の継ぎ口",
            Self::背面 => "背面の継ぎ口",
            Self::左面 => "左面の継ぎ口",
            Self::右面 => "右面の継ぎ口",
        })
    }

    fn 呼び名(self) -> &'static str {
        match self {
            Self::正面 => "正面",
            Self::背面 => "背面",
            Self::左面 => "左面",
            Self::右面 => "右面",
        }
    }
}

impl fmt::Display for 升目の側面 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.呼び名())
    }
}
