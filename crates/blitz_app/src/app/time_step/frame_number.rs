//! 起動から数えた描画機会の番号。担当するのは「今何番目の描画か」を1つ持ち、番号どうしの前後と総フレーム数との
//! 位置関係に答えることだけである。番号を進めるのは時間進行の配線であり、この型は誰が進めるかを知らない。
//!
//! 裸の整数で持たないのは、フレーム番号・総フレーム数・進める刻み数がどれも1フレーム実行の同じ場所を流れ、
//! 番号(序数)と本数(基数)を足し合わせても署名が通るためである。番号と本数の比較はこの型のメソッドだけが行い、
//! 呼び出し側が`+ 1 ==`の形で番号と本数を並べる算術を書かない。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断11」。

use std::fmt;

/// 起動から数えた描画機会の番号。最初の描画が0である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct フレーム番号(u32);

impl フレーム番号 {
    pub(crate) fn 起動時の最初() -> Self {
        Self(0)
    }

    pub(in crate::app) fn 次へ進める(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub(in crate::app) fn 最初のフレームか(self) -> bool {
        self.0 == 0
    }

    /// 奇数番目の描画か。1フレームおきに往復させる探査が向きを決めるために読む。
    pub(in crate::app) fn 奇数番目か(self) -> bool {
        self.0 % 2 == 1
    }

    /// この番号を描き終えると総フレーム数に達するか。フレーム数の決まった実行の最後の描画がこれである。
    pub(in crate::app) fn 最後のフレームか(self, 総フレーム数: u32) -> bool {
        self.0.saturating_add(1) == 総フレーム数
    }

    /// 最後のフレームの1つ前か。同一起動内の再現性を見る先行ダンプがこの描画を撮る。
    pub(in crate::app) fn 最後の一つ前のフレームか(self, 総フレーム数: u32) -> bool {
        self.0.saturating_add(2) == 総フレーム数
    }

    /// 最後の`本数`フレームの中に入るか。本数が総フレーム数を超えるときは全フレームが入る。
    pub(in crate::app) fn 最後の何フレームかに入るか(self, 総フレーム数: u32, 本数: u32) -> bool {
        self.0.saturating_add(本数.min(総フレーム数)) >= 総フレーム数
    }

    /// 進めた番号が総フレーム数に達したか。達した描画機会でイベントループを閉じる。
    pub(in crate::app) fn 総フレーム数に達したか(self, 総フレーム数: u32) -> bool {
        self.0 >= 総フレーム数
    }

    /// 同じ長さの区切りを繰り返すとき、この番号が何番目の区切りに入るか。段差の走査が撮影の番号を引く。
    pub(in crate::app) fn 区切りの何番目か(self, 区切りの長さ: u32) -> u32 {
        self.0 / 区切りの長さ
    }

    /// 同じ長さの区切りを繰り返すとき、この番号がその区切りの最後のフレームか。
    pub(in crate::app) fn 区切りの最後のフレームか(self, 区切りの長さ: u32) -> bool {
        self.0 % 区切りの長さ == 区切りの長さ.saturating_sub(1)
    }

    /// 境界向けの生値取り出し。`blitz_engine`の画素内ずらしの列・スモークの計画・ストリーミングの固定経路のように、
    /// 番号を算術で使う外の口へ渡すときだけ使う。
    pub(in crate::app) fn 境界用の生値(self) -> u32 {
        self.0
    }
}

impl fmt::Display for フレーム番号 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
