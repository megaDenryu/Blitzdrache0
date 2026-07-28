//! 1回の可視判定が数えた件数。候補数・個体判定数・可視数の3つを別々に持つ。
//! 個体判定数を候補数と別に持つのは、粗い判定で群ごと棄却したときに個体の判定が1回も走らないことを、
//! 呼び出し元と検査が観測できるようにするためである。2段階の粗い判定はこの数が0になることでしか確かめられない。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct 可視判定計数 {
    候補数: usize,
    個体判定数: usize,
    可視数: usize,
}

impl 可視判定計数 {
    /// 群の包囲領域が視錐台の外だったとき。個体の判定は1回も行わない。
    pub(super) fn 群を棄却した(候補数: usize) -> Self {
        Self {
            候補数,
            個体判定数: 0,
            可視数: 0,
        }
    }

    pub(super) fn 個体を判定した(候補数: usize, 可視数: usize) -> Self {
        Self {
            候補数,
            個体判定数: 候補数,
            可視数,
        }
    }

    pub fn 候補数(&self) -> usize {
        self.候補数
    }

    pub fn 個体判定数(&self) -> usize {
        self.個体判定数
    }

    pub fn 可視数(&self) -> usize {
        self.可視数
    }

    /// 複数の群の判定結果を1フレーム分へ合算する。
    pub fn 足す(self, 他: Self) -> Self {
        Self {
            候補数: self.候補数 + 他.候補数,
            個体判定数: self.個体判定数 + 他.個体判定数,
            可視数: self.可視数 + 他.可視数,
        }
    }
}
