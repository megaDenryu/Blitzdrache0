//! その描画で進める固定刻みの本数。担当するのは「本数を1つ持ち、数え上げと上限の比較に答える」ことだけであり、
//! 何本進めるかの決め方も、刻みの中で何をするかも知らない。
//!
//! 裸の整数で持たないのは、フレーム番号・上限の本数・刻みの秒数がどれも1フレーム実行の同じ場所を流れるためである。
//! 16ビットで持つのは、上限が実測で動かしても一桁の値であり、この本数から刻みの秒数を作るときに
//! 情報を落とさずf32へ写せる幅だからである。
//! 参照: `_doc/設計/時間の規律.md`「判断3」。

/// その描画で進める固定刻みの本数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(in crate::app) struct 進める刻み数(u16);

impl 進める刻み数 {
    pub(in crate::app) fn 生成する(本数: u16) -> Self {
        Self(本数)
    }

    /// 刻みを1本も進めない描画。蓄積が基本刻みに届かなかったときの答えである。
    pub(in crate::app) fn 一つも進めない() -> Self {
        Self(0)
    }

    /// 必ず1本だけ進める描画。フレーム数の決まった実行が毎描画で答える値である。
    pub(in crate::app) fn 一つだけ進める() -> Self {
        Self(1)
    }

    /// 境界向けの生値取り出し。刻みを回す繰り返しの回数にだけ使う。
    pub(in crate::app) fn 本数(self) -> u16 {
        self.0
    }

    /// 本数ぶんの倍率。基本刻みへ掛けてその描画で進む時間を作る。
    pub(in crate::app) fn 倍率へ写す(self) -> f32 {
        f32::from(self.0)
    }

    pub(in crate::app) fn 上限に達したか(self, 上限: Self) -> bool {
        self.0 >= 上限.0
    }

    pub(in crate::app) fn 一つ増やす(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}
