//! 1回の実行から採れた値と、生値ファイルへの書き出し。担当するのは値の形と、順序を落とさずに残すことである。
//!
//! フレーム別の値をそのまま残すのは、窓の集約(平均・p50・p95・件数)から元の列を復元できないためである。
//! 集約だけを残すと、報告した分位を後から独立に計算し直して監査する道が閉じる。
//! 周回番号と順序位置を各行へ付けるのは、順序の効果が値に残っていないかを読み手が確かめられるようにするためである。
//!
//! 書き出しは2つのファイルへ分ける。フレーム別の生値は`raw.tsv`、窓の集約は`window.tsv`である。
//! 1つのファイルに混ぜると、1行が持つ意味(1フレームの値か、窓の集約か)を列の中身から読み分けることになる。

mod write;

pub(super) use write::{生値を書く, 窓の集約を書く};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct 区間の分布 {
    pub(super) 平均ミリ秒: f64,
    pub(super) p50ミリ秒: f64,
    pub(super) p95ミリ秒: f64,
    pub(super) 標本数: u64,
}

impl 区間の分布 {
    pub(super) fn 標本数を数へ(self) -> usize {
        usize::try_from(self.標本数).unwrap_or(usize::MAX)
    }
}

/// その条件でその区間が観測されたかどうか。深度プリパスを積まない条件では、プリパスと合計の区間がそもそも立たない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum 区間の観測 {
    /// その条件はその区間を積まない。値が無いことがその条件の性質である。
    積まない,
    測れた(区間の分布),
}

/// フレーム1枚ぶんの1区間の値。窓へ入る前の生の値である。
#[derive(Debug, Clone, PartialEq)]
pub(super) struct フレーム別の生値 {
    pub(super) 読み取り順: u64,
    pub(super) 区間名: String,
    pub(super) 経過ミリ秒: f64,
}

pub(super) struct 一標本 {
    /// 起動した順の通し番号。0から始まり、回転の並びがそのまま並ぶ。
    pub(super) 実行番号: usize,
    pub(super) 周回番号: usize,
    pub(super) 順序位置: usize,
    pub(super) 条件名: &'static str,
    /// 添字が`intervals::全区間一覧`の添字である。
    pub(super) 区間別: Vec<区間の観測>,
    /// 報告する側が分位を採った窓の長さ。報告の見出しから読んだ値であり、こちらの台帳ではない。
    pub(super) 窓の標本数: usize,
    /// 読み取り順に並んだフレーム別の値。全区間ぶんが混ざった1本の列である。
    pub(super) フレーム別の値一覧: Vec<フレーム別の生値>,
}

impl 一標本 {
    /// 標本の中身が並びの前提どおりかを確かめる検査だけが読む。表と生値は並びごと走査する。
    #[cfg(test)]
    pub(super) fn 区間を参照する(&self, 区間名: &str) -> Option<区間の観測> {
        let 添字 = super::intervals::全区間一覧.iter().position(|名前| *名前 == 区間名)?;
        self.区間別.get(添字).copied()
    }
}
