//! 採取1回の由来ファイル。担当するのは「契約の行・構築の由来・採取の旗・構図の実測を1つのファイルへ
//! 書くこと」と「判定の側がその旗を読み戻すこと」である。受け取るのは採取条件と構築の由来と構図の実測、
//! 返すのは書き込みの成否、または読み戻した旗である。
//!
//! 旗の綴りを書く側と読む側で1つのモジュールへ閉じるのは、判定が「後処理なしの対」のつもりで後処理ありの
//! 絵を突き合わせても、綴りがずれていれば誰も気づかないためである。旗の記録が無ければ、判定は自分が
//! どの構成の絵を見ているかを絵からは知りようがない。

use std::path::{Path, PathBuf};

use super::error::遠景構図の検収エラー;
use super::plan::採取条件;

const 旗の行の前置き: &str = "capture-flags ";

/// 採取の起動条件のうち、判定が絵の突き合わせ方を決めるために読み戻すもの。
#[derive(Debug, PartialEq, Eq)]
pub(super) struct 採取の旗 {
    局所可視性を使うか: bool,
    遠景の影を使うか: bool,
    後処理を使うか: bool,
}

impl 採取の旗 {
    pub(super) fn 採取条件から作る(条件: &採取条件) -> Self {
        Self {
            局所可視性を使うか: !条件.ssaoを使わない,
            遠景の影を使うか: !条件.遠景影を使わない,
            後処理を使うか: !条件.後処理を使わない,
        }
    }

    pub(super) fn 後処理を使うか(&self) -> bool {
        self.後処理を使うか
    }

    fn 行にする(&self) -> String {
        format!(
            "{旗の行の前置き}ssao={} distant-shadow={} post={}",
            入切(self.局所可視性を使うか),
            入切(self.遠景の影を使うか),
            入切(self.後処理を使うか)
        )
    }

    /// 名前の由来ファイルから旗の行を1本読む。行が無いのは古い採取であり、判定が
    /// 条件を確かめられないまま通ることを防ぐため、値の食い違いと同じく失敗にする。
    pub(super) fn 由来ファイルから読む(置き場: &Path, 名前: &str) -> Result<Self, String> {
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
        })
    }
}

pub(super) fn 由来を書く(
    置き場: &Path,
    条件: &採取条件,
    由来: &crate::release_build::構築の由来,
    構図の実測: &str,
) -> Result<(), 遠景構図の検収エラー> {
    let パス = 置き場.join(format!("{}.txt", 条件.名前));
    let mut 行一覧 = vec![format!("distant-view-contract=v1 {}", super::plan::計画を表示する())];
    行一覧.push(採取の旗::採取条件から作る(条件).行にする());
    行一覧.extend(由来.注記一覧());
    行一覧.push(format!("view-measurement={構図の実測}"));
    std::fs::write(&パス, 行一覧.join("\n")).map_err(|誤り| 遠景構図の検収エラー::由来を書けなかった {
        パス: PathBuf::from(&パス),
        誤り,
    })
}

fn 入切(使うか: bool) -> &'static str {
    if 使うか { "on" } else { "off" }
}

fn 入切を読む(本文: &str, 前置き: &str) -> Result<bool, String> {
    let 綴り = 本文
        .split_whitespace()
        .find_map(|語| 語.strip_prefix(前置き))
        .ok_or_else(|| format!("採取の旗に{前置き}が無い: {本文}"))?;
    match 綴り {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(format!("採取の旗の{前置き}がonでもoffでもない: {綴り}")),
    }
}
