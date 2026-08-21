//! 升目の座標: ベイ格子の中で1つの升目を指す位置であり、それ自体が升目の識別子である。
//!
//! 3つの軸を裸の整数の組で持たないのは、横と奥を取り違えても型が通り、その取り違えが「建物の形が違う」という
//! 遠い場所の失敗になるためである。チャンク座標が同じ理由で2成分を包んでいる。
//!
//! 軸の向きは骨格の側面の宣言が決めている。横は右面の法線の向き(部品ローカルのxが増える向き)へ、
//! 奥は背面の法線の向き(部品ローカルのzが減る向き)へ増える。階は0から数え、0が最も下の階である。
//! 参照: `_doc/設計/部品カタログと接合点.md`「骨格の上下積層とベイの継ぎ口(2026-08-19 追加)」

use std::fmt;

use super::side_face::升目の側面;

/// 順序は横を先、奥を次、階を最後に見る辞書式である。並べ替えの規則を型が持つのは、座標を鍵にした表を
/// 走査する側が、同じ格子から常に同じ並びを得るためである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 升目の座標 {
    横: i32,
    奥: i32,
    階: u32,
}

impl 升目の座標 {
    /// 3つの軸からのみ作れるため、どの升目も指さない座標を作る経路が存在しない。
    pub const fn 生成する(横: i32, 奥: i32, 階: u32) -> Self {
        Self { 横, 奥, 階 }
    }

    pub fn 横(self) -> i32 {
        self.横
    }

    pub fn 奥(self) -> i32 {
        self.奥
    }

    pub fn 階(self) -> u32 {
        self.階
    }

    /// 同じ階で名指した側面の向こう隣にある升目の座標。整数の端で溢れるときは隣が無いものとして答える。
    pub(super) fn 同じ階の隣(self, 側面: 升目の側面) -> Option<Self> {
        let (横の進み, 奥の進み) = match 側面 {
            升目の側面::正面 => (0, -1),
            升目の側面::背面 => (0, 1),
            升目の側面::左面 => (-1, 0),
            升目の側面::右面 => (1, 0),
        };
        Some(Self {
            横: self.横.checked_add(横の進み)?,
            奥: self.奥.checked_add(奥の進み)?,
            階: self.階,
        })
    }

    /// 真下の升目の座標。最も下の階には真下が無い。
    pub(super) fn 真下の升目(self) -> Option<Self> {
        Some(Self {
            階: self.階.checked_sub(1)?,
            ..self
        })
    }

    pub(super) fn 真上の升目(self) -> Option<Self> {
        Some(Self {
            階: self.階.checked_add(1)?,
            ..self
        })
    }
}

impl fmt::Display for 升目の座標 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "横の番号{}・奥の番号{}・階の番号{}", self.横, self.奥, self.階)
    }
}
