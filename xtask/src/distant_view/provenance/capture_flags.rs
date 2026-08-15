//! 採取1回の起動条件を、由来ファイルの1行として書き、判定の側が同じ綴りで読み戻す工程。
//! 受け取るのは採取条件または由来ファイルの置き場と名前、返すのは旗の行または読み戻した旗である。
//!
//! 書く側と読む側で綴りを1つのモジュールへ閉じるのは、判定が「後処理なしの対」のつもりで後処理ありの絵を
//! 突き合わせても、綴りがずれていれば誰も気づかないためである。旗の記録が無ければ、判定は自分がどの構成の絵を
//! 見ているかを絵からは知りようがない。

mod on_off;

use on_off::{入切, 入切を読む, 記録があれば入切を読む};

use std::path::Path;

use super::super::plan::採取条件;

const 旗の行の前置き: &str = "capture-flags ";

/// 採取の起動条件のうち、判定が絵の突き合わせ方を決めるために読み戻すもの。
#[derive(Debug, PartialEq, Eq)]
pub(in crate::distant_view) struct 採取の旗 {
    局所可視性を使うか: bool,
    遠景の影を使うか: bool,
    後処理を使うか: bool,
    /// 影可視度の可視化の記録。この欄が無い由来は、欄を足す前に採った古い採取である。
    /// 欄の有無で失敗させるのは、その欄を実際に要る段(影の検査点)だけにする。遠景の検査点は影可視度を読まないため、
    /// 古い採取のままでも判定を続けられる。無いことを`false`へ丸めると、診断の絵を本番の絵として突き合わせうる。
    影可視度を可視化したか: Option<bool>,
    /// 距離区分の再配分を切って走ったかの記録。この欄が無い由来は、欄を足す前に採った古い採取である。
    明示境界を切ったか: Option<bool>,
}

impl 採取の旗 {
    pub(in crate::distant_view) fn 採取条件から作る(条件: &採取条件) -> Self {
        Self {
            局所可視性を使うか: !条件.ssaoを使わない,
            遠景の影を使うか: !条件.遠景影を使わない,
            後処理を使うか: !条件.後処理を使わない,
            影可視度を可視化したか: Some(条件.影可視度を可視化する),
            明示境界を切ったか: Some(条件.明示境界を使わない),
        }
    }

    pub(in crate::distant_view) fn 後処理を使うか(&self) -> bool {
        self.後処理を使うか
    }

    /// 影可視度の可視化の記録。欄を持たない古い採取では`None`であり、読む側が採り直しを促す。
    pub(in crate::distant_view) fn 影可視度を可視化したか(&self) -> Option<bool> {
        self.影可視度を可視化したか
    }

    /// 距離区分の再配分を切って走ったかの記録。欄を持たない古い採取では`None`である。
    pub(in crate::distant_view) fn 明示境界を切ったか(&self) -> Option<bool> {
        self.明示境界を切ったか
    }

    pub(super) fn 行にする(&self) -> String {
        format!(
            "{旗の行の前置き}ssao={} distant-shadow={} post={} shadow-visibility={} no-explicit-bands={}",
            入切(self.局所可視性を使うか),
            入切(self.遠景の影を使うか),
            入切(self.後処理を使うか),
            self.影可視度を可視化したか.map_or("unrecorded", 入切),
            self.明示境界を切ったか.map_or("unrecorded", 入切)
        )
    }

    /// 名前の由来ファイルから旗の行を1本読む。行が無いのは古い採取であり、判定が
    /// 条件を確かめられないまま通ることを防ぐため、値の食い違いと同じく失敗にする。
    pub(in crate::distant_view) fn 由来ファイルから読む(置き場: &Path, 名前: &str) -> Result<Self, String> {
        let パス = 置き場.join(format!("{名前}.txt"));
        let 内容 = std::fs::read_to_string(&パス).map_err(|誤り| format!("{}を読めない: {誤り}", パス.display()))?;
        let 本文 = 内容
            .lines()
            .find_map(|行| 行.strip_prefix(旗の行の前置き))
            .ok_or_else(|| format!("{}に採取の旗の行が無い。この名前を採り直す必要がある", パス.display()))?;
        Ok(Self {
            局所可視性を使うか: 入切を読む(本文, "ssao=")?,
            遠景の影を使うか: 入切を読む(本文, "distant-shadow=")?,
            後処理を使うか: 入切を読む(本文, "post=")?,
            影可視度を可視化したか: 記録があれば入切を読む(本文, "shadow-visibility=")?,
            明示境界を切ったか: 記録があれば入切を読む(本文, "no-explicit-bands=")?,
        })
    }
}
